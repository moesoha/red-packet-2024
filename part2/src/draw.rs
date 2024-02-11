lazy_static::lazy_static! {
	pub static ref HONGBAO: Vec<u8> = {
		let a = vec![
			"\x1b[H\x1b[J\x1b[H",
			"     \x1b[30;41m|                            |\x1b[m",
			"     \x1b[30;41m \\                          / \x1b[m",
			"     \x1b[30;41m  \\                        /  \x1b[m",
			"     \x1b[30;41m   \\                      /   \x1b[m",
			"     \x1b[30;41m    \\                    /    \x1b[m",
			"     \x1b[30;41m     \\__________________/     \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[33;41m           /-\x1b[37;43m    \x1b[33;41m-\\           \x1b[m",
			"     \x1b[33;41m           \x1b[37;43m ------ \x1b[33;41m           \x1b[m",
			"     \x1b[33;41m           \x1b[37;43m -|--|- \x1b[33;41m           \x1b[m",
			"     \x1b[33;41m           \x1b[37;43m  /  |  \x1b[33;41m           \x1b[m",
			"     \x1b[33;41m           \\-\x1b[37;43m    \x1b[33;41m-/           \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			""
		];
		let a= a.join("\r\n");
		a.as_bytes().to_vec()
	};

	pub static ref CODE_HONGBAO: Vec<u8> = b"51131748".to_vec();

	pub static ref HONGBAO_EASTER_OPENED: Vec<u8> = {
		let a = vec![
			"\x1b[H\x1b[J\x1b[H",
			"        \x1b[47m                        \x1b[m",
			"        \x1b[30;47m    !CONGRATULATION!    \x1b[m",
			"        \x1b[47m                        \x1b[m",
			"        \x1b[30;47m    You've found the    \x1b[m",
			"        \x1b[47m      \x1b[40m \x1b[31mE\x1b[32mA\x1b[33mS\x1b[34mT\x1b[35mE\x1b[36mR \x1b[37mEGG \x1b[47m      \x1b[m",
			"        \x1b[47m                        \x1b[m",
			"        \x1b[37;47m     \x1b[41m Hongbao Code \x1b[47m     \x1b[m",
			"        \x1b[31;47m        58420229        \x1b[m",
			"        \x1b[47m                        \x1b[m",
			"        \x1b[47m                        \x1b[m",
			"",
			"          \x1b[37m____________________\x1b[m",
			"         \x1b[37m/\x1b[41m                    \x1b[m\x1b[37m\\\x1b[m",
			"        \x1b[37m/\x1b[41m                      \x1b[m\x1b[37m\\\x1b[m",
			"       \x1b[37m/\x1b[41m                        \x1b[m\x1b[37m\\\x1b[m",
			"      \x1b[37m/\x1b[41m                          \x1b[m\x1b[37m\\\x1b[m",
			"     \x1b[37m|\x1b[30;41m____________________________\x1b[m\x1b[37m|\x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			"     \x1b[41m                              \x1b[m",
			""
		];
		let a= a.join("\r\n");
		a.as_bytes().to_vec()
	};

	pub static ref PROMPT_UNOPENED: Vec<u8> = {
		let a = vec![
			"\x1b[22H\x1b[m",
			"  I WILL give you hongbao,",
			"  but you did not ask me for that!",
		];
		let a= a.join("\r\n");
		a.as_bytes().to_vec()
	};

	pub static ref PROMPT_TIMEOUT: Vec<u8> = b"\r\n\r\n\r\nOperation Timeout. Disconnecting.\r\n\r\n".to_vec();

	pub static ref PROMPT_OPENED: Vec<u8> = {
		let a = vec![
			"\x1b[22H\x1b[m",
			"  CONGRATULATION!",
			"  You have opened your hongbao!"
		];
		let a= a.join("\r\n");
		a.as_bytes().to_vec()
	};

	pub static ref PROMPT_UNOPENED_EASTER_OPENED: Vec<u8> = {
		let a = vec![
			"\x1b[22H\x1b[m",
			"  You find the easter egg, but still",
			"  not ask me for that I WILL give you."
		];
		let a= a.join("\r\n");
		a.as_bytes().to_vec()
	};
}
