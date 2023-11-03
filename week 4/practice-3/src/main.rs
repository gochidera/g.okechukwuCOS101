fn main() {
    let name1 = "Gloria Okechukwu";
    println!("My name is {}",name1);

    //find and repalce
    let name2 = name1.replace("Gloria","Chidera");
    println!("You can also call me {}",name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am the student of the {}", school);
}
