use std::io;

fn main() {
	
	println!("Menu			      Price
P = Poundo Yam/Edinkaiko Soup 	- N3,200
F = Fried Rice & Chicken 		- N3,000
A = Amala & Ewedu Soup			- N2,500
E = Eba & Egusi Soup			- N2,000
W = White Rice & Stew			- N2,500
");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("How many portions of poundo yam and edinkaiko do you want");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("How many portions of rice and chicken do you want");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("How many portions of amala and ewedu soup do you want");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    println!("How many portions of eba and egusi soup do you want");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let d:f64 = input4.trim().parse().expect("Not a valid number");

    println!("How many portions of white rice and stew do you want");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let e:f64 = input5.trim().parse().expect("Not a valid number");

    let p:f64 = 3200.0;
    let f:f64 = 3000.0;
    let a:f64 = 2500.0;
    let e:f64 = 2000.0;
    let w:f64 = 2500.0;

    let p = a * p;

    let f = b * f;

    let a = c * a;

    let e = d * e;

    let w = e * w;

    let bill = p + f + a + e + w;

    if bill > 10000.0 {
    	let discount = 0.95 * bill;
    	println!("\nYou have a discount of 5% and your bill is {}",discount);
    }


}
