use std::io;
use std::io::ErrorKind;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use tokio::net::TcpStream;
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder, Framed};

pub type TelnetStream = Framed<TcpStream, Telnet>;
pub fn new_telnet_stream(stream: TcpStream) -> TelnetStream {
	Framed::new(stream, Telnet)
}

pub struct Telnet;
impl Encoder<TelnetData> for Telnet {
	type Error = io::Error;

	fn encode(&mut self, item: TelnetData, dst: &mut BytesMut) -> Result<(), Self::Error> {
		const ESCAPE: u8 = TelnetCommand::Escape as u8;
		match item {
			TelnetData::Binary(v) => {
				let mut buf = &v[..];
				while let Some(n) = buf.iter().position(|&b| b == ESCAPE) {
					dst.extend_from_slice(&buf[..=n]);
					dst.put_u8(ESCAPE);
					buf = &buf[(n + 1)..];
				}
				dst.extend_from_slice(buf);
				return Ok(()) // FIXME
			},
			TelnetData::SubnegotiationRequest(o, v) => {
				dst.extend_from_slice(&[ESCAPE, TelnetCommand::Subnegotiation as u8, o.into()]);
				dst.extend_from_slice(&v); // FIXME: 0xff escape
				dst.extend_from_slice(&[ESCAPE, TelnetCommand::SubnegotiationEnd as u8]);
			},
			TelnetData::Will(true, o) => { dst.extend_from_slice(&[ESCAPE, TelnetCommand::Will as u8, o.into()]) },
			TelnetData::Will(false, o) => { dst.extend_from_slice(&[ESCAPE, TelnetCommand::WillNot as u8, o.into()]) },
			TelnetData::Do(true, o) => { dst.extend_from_slice(&[ESCAPE, TelnetCommand::Do as u8, o.into()]) },
			TelnetData::Do(false, o) => { dst.extend_from_slice(&[ESCAPE, TelnetCommand::DoNot as u8, o.into()]) },
			TelnetData::Command(c) => { dst.extend_from_slice(&[ESCAPE, c as u8]); },
			TelnetData::SubnegotiationResponse(_) => ()
		}
		Ok(())
	}
}
impl Decoder for Telnet {
	type Item = TelnetData;
	type Error = io::Error;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		const ESCAPE: u8 = TelnetCommand::Escape as u8;
		if src.len() == 0 { return Ok(None); }
		let mut buf: Vec<u8> = vec![];
		let mut stop_before: Option<usize> = None;
		while let Some(n) = src.iter().position(|&b| b == ESCAPE) {
			let next = match src.get(n + 1).map(|&x| TelnetCommand::try_from(x).ok()) {
				Some(Some(b)) => b,
				Some(None) => continue,
				None => return Ok(None)
			};
			match next {
				c @ (TelnetCommand::Escape|TelnetCommand::Nop|TelnetCommand::DataMark|TelnetCommand::SubnegotiationEnd) => {
					buf.extend_from_slice(&src[..n]);
					if c == TelnetCommand::Escape {
						buf.push(0xff);
					}
					src.advance(n + 2);
					continue;
				},
				_ => if n != 0 {
					stop_before = Some(n);
					break;
				}
			}
			match next {
				TelnetCommand::Escape|TelnetCommand::Nop|TelnetCommand::DataMark|TelnetCommand::SubnegotiationEnd => panic!("should not happen"),
				c @ (
					TelnetCommand::GoAhead|TelnetCommand::AreYouThere|
					TelnetCommand::EraseLine|TelnetCommand::EraseCharacter|
					TelnetCommand::AbortOutput|TelnetCommand::InterruptProcess|TelnetCommand::Break
				) => {
					src.advance(2);
					return Ok(Some(TelnetData::Command(c)));
				}
				c @ (TelnetCommand::Will|TelnetCommand::WillNot|TelnetCommand::Do|TelnetCommand::DoNot) => {
					return Ok(if let Some(option) = src.get(2).map(|&x| TelnetOption::from(x)) {
						src.advance(3);
						Some(match c {
							TelnetCommand::Will => TelnetData::Will(true, option),
							TelnetCommand::WillNot => TelnetData::Will(false, option),
							TelnetCommand::Do => TelnetData::Do(true, option),
							TelnetCommand::DoNot => TelnetData::Do(false, option),
							_ => panic!("should not happen")
						})
					} else {
						None
					});
				},
				TelnetCommand::Subnegotiation => {
					let option = if let Some(o) = src.get(2).map(|&x| TelnetOption::from(x)) {
						o
					} else {
						return Ok(None);
					};
					let mut fin = None;
					let (start, mut count) = (3, 4); // IAC, SB, <OPT>, and skip one more since there should be an IAC before SE
					let mut buf = &src[count..];
					while let Some(n) = buf.iter().position(|&b| b == (TelnetCommand::SubnegotiationEnd as u8)) {
						count += n;
						let mut iac_before = 0usize;
						for &b in buf[..n].iter().rev() {
							if b == ESCAPE {
								iac_before += 1;
							} else {
								break;
							}
						}
						if iac_before % 2 == 1 { // double IAC is 0xff
							fin = Some(src[start..(count - 1)].to_vec());
							break;
						}
						buf = &buf[(n + 1)..];
					}
					if fin.is_none() { return Ok(None); }
					src.advance(count + 1);
					return Ok(fin.unwrap()).and_then(|mut v| {
						let mut last = 0;
						while let Some(n) = &v[last..].iter().position(|&x| x == ESCAPE) {
							if let Some(&ESCAPE) = v.get(last + n + 1) {
								v.remove(last + n);
							}
							last += *n + 1;
						}
						TelnetSubnegotiationResponse::new(option, v)
							.map(|r| Some(TelnetData::SubnegotiationResponse(r)))
					});
				}
			}
		}
		// rest are telnet data
		let stop_len = stop_before.unwrap_or(src.len());
		buf.extend_from_slice(&src[..stop_len]);
		src.advance(stop_len);
		Ok(Some(TelnetData::Binary(buf)))
	}
}

#[derive(Debug)]
pub enum TelnetData {
	Command(TelnetCommand),
	Will(bool, TelnetOption),
	Do(bool, TelnetOption),
	SubnegotiationRequest(TelnetOption, Vec<u8>),
	SubnegotiationResponse(TelnetSubnegotiationResponse),
	Binary(Vec<u8>)
}

#[derive(Debug)]
pub enum TelnetSubnegotiationResponse {
	TerminalType(String),
	WindowSize { col: u16, row: u16 },
	Any(TelnetOption, Vec<u8>)
}
impl TelnetSubnegotiationResponse {
	pub fn new(option: TelnetOption, mut data: Vec<u8>) -> Result<Self, io::Error> {
		match option {
			TelnetOption::TerminalType => {
				let code = data.remove(0);
				if code != 0 {
					return Err(io::Error::new(ErrorKind::InvalidData, "unexpected TerminalType response"));
				}
				String::from_utf8(data)
					.map_err(|_| io::Error::new(ErrorKind::InvalidData, "unexpected utf8 string"))
					.map(|s| Self::TerminalType(s))
			},
			TelnetOption::WindowSizeNegotiation => {
				if data.len() != 4 {
					return Err(io::Error::new(ErrorKind::InvalidData, "unexpected WindowSize response"));
				}
				Ok(Self::WindowSize {
					col: u16::from_be_bytes([data[0], data[1]]),
					row: u16::from_be_bytes([data[2], data[3]])
				})
			}
			o => Ok(Self::Any(o, data))
		}
	}
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum TelnetCommand {
	Escape = 255,
	DoNot = 254,
	Do = 253,
	WillNot = 252,
	Will = 251,
	Subnegotiation = 250,
	GoAhead = 249,
	EraseLine = 248,
	EraseCharacter = 247,
	AreYouThere = 246,
	AbortOutput = 245,
	InterruptProcess = 244,
	Break = 243,
	DataMark = 242,
	Nop = 241,
	SubnegotiationEnd = 240
}
impl TelnetCommand {
	pub fn is_command(&self) -> bool {
		match self {
			Self::GoAhead|Self::AreYouThere|
			Self::EraseLine|Self::EraseCharacter|
			Self::AbortOutput|Self::InterruptProcess|Self::Break => true,
			_ => false
		}
	}
}

#[derive(Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TelnetOption {
	BinaryTransmission = 0,
	Echo = 1,
	Reconnection = 2,
	SuppressGoAhead = 3,
	ApproxMessageSizeNegotiation = 4,
	Status = 5,
	TimingMark = 6,
	RemoteControlledTransAndEcho = 7,
	ExtendedAscii = 17,
	TerminalType = 24,
	WindowSizeNegotiation = 31,
	TerminalSpeed = 32,
	RemoteFlowControl = 33,
	LineMode = 34,
	XDisplayControl = 35,
	Environment = 36,
	Authentication = 37,
	Encryption = 38,
	NewEnvironment = 39,
	Charset = 42,
	// #[num_enum(alternatives=[50..=137, 141..=253])] // 254 is used by unknown
	// Unassigned(u8) = 253,
	Hongbao2024 = 240,
	ExtendedOptionsList = 255,
	#[num_enum(catch_all)]
	Unknown(u8) = 254
}
