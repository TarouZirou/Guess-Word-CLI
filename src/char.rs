use char_kind::{EXIST, MATCH, WRONG};
use rand::Rng;

#[test]
fn show_chars() {
	let mut vec: Vec<u8> = Vec::new();
	for i in 0..26 {
		vec.push(b'a' + i);
	}

	println!("{:?}", vec);
	println!("{}", unsafe { String::from_utf8_unchecked(vec) });

	vec = Vec::new();
	for i in 0..26 {
		vec.push(b'A' + i);
	}
	println!("{:?}", vec);
	println!("{}", unsafe { String::from_utf8_unchecked(vec) });

	for _ in 0..10 {
		println!("{}", mk_chars().iter().collect::<String>());
	}
}

pub type CharColor = usize;
pub mod char_kind {
	pub const WRONG: super::CharColor = 0;
	pub const EXIST: super::CharColor = 1;
	pub const MATCH: super::CharColor = 2;
}

pub enum CharKind {
	WRONG,
	EXIST,
	MATCH,
}

pub const COLOR_TABLE: [&str; 3] = ["\x1b[34m", "\x1b[33m", "\x1b[32m"];
///0~25の乱数をアルファベットに変換、それを5回繰り返して、文字列を返す関数
pub fn mk_chars() -> Vec<char> {
	let mut rng = rand::rng();
	let rand_chars: Vec<char> = (0..5)
		.map(|_| (b'A' + rng.random_range(0..26)) as char)
		.collect();
	rand_chars
}
