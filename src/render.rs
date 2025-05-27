use crate::char::{char_kind, mk_chars, COLOR_TABLE};

///TODO: 入力した文字列に、判定結果の色を加える関数
pub fn render(answer: &Vec<char>, input: &Vec<char>) {}

#[test]
fn color_print() {
	let chars = mk_chars();
	let strings: Vec<String> = chars.iter().map(|c| c.to_string()).collect();
	let chars2 = mk_chars();
	let strings2: Vec<String> = chars.iter().map(|c| c.to_string()).collect();
	println!("{:?}", chars);
	println!("{:?}", chars2);

	let mut check: Vec<String> = Vec::new();
	for s in 0..5 {
		if &chars[s] == &chars2[s] {
			check.push(String::from(format!(
				"{}{}\x1b[0m",
				COLOR_TABLE[char_kind::MATCH],
				&chars[s]
			)));
		} else {
			check.push(chars[s].to_string());
		}
	}
	println!("{}", check.join(""));
}
