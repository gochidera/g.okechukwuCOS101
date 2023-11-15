// Rust program to determine the incentive of an employee
//using their age and experience

use std::io;

fn main()
{
	let mut input2 = String::new();
	let mut age = String::new();

    println!("Are you experienced or inexperienced:");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let e = input2.trim();

    println!("Enter employee's age:");
    io::stdin().read_line(&mut age).expect("Not a valid number");
    let age:f64 = age.trim().parse().expect("Failed to input");

    if e == "experienced" && age >= 40.0
    {
    	println!("incentive is 1560000");
    }
    else if e == "experienced" && age >= 30.0 && age < 40.0
    {
    	println!("incentive is 1480000");
    }
    else if e == "experienced" && age < 28.0
    {
    	println!("incentive is 15600000");
    }
    else if e == "inexperienced"
    { 
    	println!("incentive is 100000.0");
    }
    	
   

    


    




}
