 use std::io::Write;

fn main() {
    
    let Drink1 = "Larger\n";

    let mut file = std::fs::File::create("Larger.txt").expect("Create File");
    file.write_all(33 Export'
    	           "Desperados"
    	           "Goldberg"
    	           "Gulder"
    	           "Heineken"
    	           "Star"
    	           .as_bytes()).expect("write failed");
    	 file.write_all(Drink1.as_bytes()).expect("Write failed");
     println!("\nData Written to file");
}
