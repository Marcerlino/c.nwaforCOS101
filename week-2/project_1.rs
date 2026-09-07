fn main() {
	let p: f64 = 520_000_000.00;
	let r: f64 = 10.0;
	let n: f64 = 5.0;

	//compound interest
	let a = p * (1.0 + ( r / 100.0 )).powf(n) ;
	println!("the amount is = \u{20a6}{}", a);
	let ci = a - p;
	println!("compound interest = \u{20a6}{}", ci);
	println!("the compound interest on N520_000_000 after 5 years at 10% per annum is = \u{20a6}{}", ci);

}