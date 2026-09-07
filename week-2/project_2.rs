fn main() {
	
	//I want to assign each name a variable 
	let _t = "tashiba";
	let _m = "mac";
	let _h = "hp";
	let _d = "dell";
	let _a = "acer";

	//To calculate the total quantity of goods sold
	let t:f64 = 2.0;
	let m:f64 = 1.0;
	let h:f64 = 3.0;
	let d:f64 = 3.0;
	let a:f64 = 1.0;
	let q = t + m + h + d + a;
	println!("the quantity sold = {}",q);

	//To calculate the sum of the total amount of goods sold
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;
    let s = t + m + h + d + a;
	println!("the sum = {}",s);

	//To calculate the average of the good solds
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;
	let average = ( t + m + h + d + a)/5.0;
	println!("the average = {}",average);
	

}