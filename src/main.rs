use rand::Rng;

//0~25の乱数をアルファベットに変換、それを5回繰り返して、文字列を返す関数
fn mk_word() -> String {
	let mut rng = rand::rng();
	let rand_chars: String = (0..5)
		.map(|_| (b'A' + rng.random_range(0..26)) as char)
		.collect();
	rand_chars
}

//TODO: 入力した文字列に、判定結果の色を加える関数
fn render() {}

//TODO: 文字列の一致を判定する関数
fn decide() {}

fn main() {
	println!("Hello, world!");

	let mut word = String::new();
	std::io::stdin().read_line(&mut word).ok();
	let answer = word.trim().to_string();

	println!("{}", answer);
}
