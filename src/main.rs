//!文字列を当てるゲーム
mod char;
mod check;
mod render;

use char::mk_chars;
use rand::Rng;

fn main() {
	let chars = mk_chars();
	println!("Guess 5 Chars!");

	loop {
		let mut word = String::new();
		std::io::stdin().read_line(&mut word).ok();
		let answer = word.trim().to_string();
		println!("{}", answer);
	}
}
