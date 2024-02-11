use std::net::ToSocketAddrs;
use std::time::Duration;
use futures::{SinkExt, stream};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use crate::telnet::{new_telnet_stream, TelnetData, TelnetOption, TelnetStream};

mod telnet;
mod draw;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    const TARGET: &str = "main";
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).env().init()?;
    let listener = TcpListener::bind(std::env::var("HB_BIND")
        .unwrap_or("[::]:23".to_string())
        .to_socket_addrs().unwrap()
        .next().unwrap()
    ).await?;
    log::info!(target: TARGET, "Server started at {:?}", listener.local_addr());

    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            log::info!(target: TARGET, "Client {:?} connected", &addr);
            let tag = format!("client({})", &addr);
            let tag_ = tag.clone();
            let tag: &str = tag.as_str();

            let mut transport = new_telnet_stream(stream);
            // Windows will not send negotiation unless receive some, send one to trigger
            transport.send(TelnetData::Will(true, TelnetOption::Echo)).await.ok();
            tokio::time::sleep(Duration::from_millis(666)).await;

            service_loop(tag_, transport).await;
            log::debug!(target: tag, "disconnecting");
        });
    }
}

async fn service_loop(tag: String, mut transport: TelnetStream) {
    let tag = tag.as_str();
    let mut option_neg_sent = false;
    let mut csi_found = None;
    let mut csi_sequence_ptr = 0usize;
    let mut csi_sequence = [0u8; 32];

    async fn render_hongbao(transport: &mut TelnetStream, status: (bool, bool)) {
        transport.send_all(&mut stream::iter(match status {
            (false, false) => [draw::HONGBAO.clone(), draw::PROMPT_UNOPENED.clone()],
            (true, false) => [draw::HONGBAO.clone(), draw::PROMPT_OPENED.clone()],
            (false, true) => [draw::HONGBAO_EASTER_OPENED.clone(), draw::PROMPT_UNOPENED_EASTER_OPENED.clone()],
            (true, true) => [draw::HONGBAO_EASTER_OPENED.clone(), draw::PROMPT_OPENED.clone()]
        }.into_iter().map(TelnetData::Binary).map(Ok))).await.ok();
    }
    let (mut opened_main, mut opened_easter) = (false, false);
    render_hongbao(&mut transport, (opened_main, opened_easter)).await;

    let timeout = tokio::time::sleep(Duration::from_secs(60));
    tokio::pin!(timeout);

    loop { tokio::select! {
        _ = &mut timeout => {
            transport.send(TelnetData::Binary(draw::PROMPT_TIMEOUT.clone())).await.ok();
            break;
        },
        result = transport.next() => match result {
            None => {
                log::debug!(target: tag, "transport closed");
                break;
            },
            Some(Err(e)) => {
                log::error!(target: tag, "framed transport error: {:?}", e);
                break;
            },
            Some(Ok(d)) => match d {
                TelnetData::Binary(buf) => {
                    const ESC: u8 = 0x1b;
                    const CSI: u8 = b'[';
                    for byte in buf {
                        // escape!
                        if byte == ESC {
                            csi_found = Some(false);
                            continue;
                        }
                        if byte == b'\r' { // when CR, ask terminal to report the cursor location
                            csi_found = None;
                            transport.send(TelnetData::Binary(vec![ESC, CSI, b'6', b'n'])).await.ok();
                            continue;
                        }
                        // No need to process non-CSI data
                        if csi_found == None { continue; }
                        if !csi_found.unwrap() {
                            csi_found = if byte == CSI {
                                csi_sequence_ptr = 0;
                                Some(true)
                            } else {
                                None
                            };
                            continue;
                        }
                        // Only CSI sequences are here
                        match byte {
                            0x20..=0x3f => { // these are params
                                csi_sequence[csi_sequence_ptr] = byte;
                                csi_sequence_ptr += 1;
                                if csi_sequence.len() == csi_sequence_ptr {
                                    csi_found = None; // too long!
                                }
                            },
                            0x40..=0x7e => { // final byte
                                csi_found = None;
                                match byte {
                                    (b'A'..=b'D') => { // echo back arrow keys
                                        transport.send(TelnetData::Binary(vec![ESC, CSI, byte])).await.ok();
                                    },
                                    b'R' => { // cursor location report
                                        if opened_easter { continue; } // nothing to do if easter is opened
                                        let loc = String::from_utf8_lossy(&csi_sequence[..csi_sequence_ptr]);
                                        let mut loc = loc.split(';');
                                        let row = loc.next().and_then(|s| str::parse::<usize>(s).ok());
                                        let col = loc.next().and_then(|s| str::parse::<usize>(s).ok());
                                        if let (Some(row), Some(col)) = (row, col) {
                                            log::info!(target: tag, "Cursor report: {},{}", row, col);
                                            if let (12..=16, 17..=24) = (row, col) {
                                                opened_easter = true;
                                                log::warn!(target: tag, "Hongbao::Easter opened!");
                                                render_hongbao(&mut transport, (opened_main, opened_easter)).await;
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            _ => csi_found = None
                        }
                    }
                },
                c@(TelnetData::Do(_, _)|TelnetData::Will(_, _)) => {
                    log::debug!(target: tag, "{:?}", c);
                    if let TelnetData::Do(true, TelnetOption::Hongbao2024) = c {
                        opened_main = true;
                        log::warn!(target: tag, "Hongbao::Main opened!");
                        transport.send(TelnetData::SubnegotiationRequest(TelnetOption::Hongbao2024, draw::CODE_HONGBAO.clone())).await.ok();
                        render_hongbao(&mut transport, (opened_main, opened_easter)).await;
                        continue;
                    }
                    if !option_neg_sent {
                        option_neg_sent = true;
                        transport.send_all(&mut stream::iter([
                            TelnetData::Do(false, TelnetOption::LineMode),
                            TelnetData::Will(true, TelnetOption::Echo),
                            TelnetData::Do(false, TelnetOption::Echo),
                            TelnetData::Will(true, TelnetOption::Hongbao2024)
                        ].into_iter().map(Ok))).await.ok();
                    }
                },
                c => { log::debug!(target: tag, "{:?}", c); }
            }
        }
    } }
}
