fn main() {


    //To calculate the depreciation of the tv set
    let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;
	let a = p * ( 1.0 - ( r / 100.00 ) ).powf(n);
	println!("the value = {}",a);
	let d = p - a;
	println!("the depreciation = {}",d);
	println!("The new value of the car after three years = {}",a);

}