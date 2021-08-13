pub fn run() {
    //Print to console
    println!("Hello from the print.rs file!");
    //To print any Integer
    println!("Number: {}", 1);
    //For Multiple PLaceholder
    println!("{} is from {}", "Asad", "India");
    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Asad", "India", "Code");
    //Named Arguments
    println!("{name} likes to write {activity}", name = "Asad", activity = "Code");
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    //Placeholder for debug traits
    println!("{:?}", (12, true, "Syed Asad"));
    //Basic Maths
    println!("10 + 10 = {}", 10+10);
}