
use std::io;

fn main() {
    
    let mut distance = String::new();
    let mut time = String::new();

    println!("Enter distance in miles: ");
    io::stdin().read_line(&mut distance).expect("Not a valid string");
    let distance:f32 = distance.trim().parse().expect("Not a valid number");

    println!("Enter time in hours: ");
    io::stdin().read_line(&mut time).expect("Not a valid string");
    let time:f32 = time.trim().parse().expect("Not a valid number");

    let distance:f32 = distance * 1.60934;

    let speed = distance / time;

    println!("The speed is {}", speed);
}
