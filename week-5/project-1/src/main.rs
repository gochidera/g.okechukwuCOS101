use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Invalid input");

    println!("Enter the value for c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("invalid input");

    let discriminant = (b * b) - 4.0 * a * c;

    if discriminant > 0.0
    {
    	println!("There are two distinct roots");
    }

    else if discriminant ==0.0
    {
    	println!("There are two distinct roots");
    }
    else if discriminant < 0.0
    {
    	println!("There are no real roots");
    }
}
