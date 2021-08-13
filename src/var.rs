//Variables hold primitive data or reference to data
//Variables are immutable by default
// Rust is a block-scoped language
//To make variable mutable use mut

pub fn run() {
    let name = "Syed Asad";
    let mut age = 24;
    println!("My Name is {} and I am {}", name, age);
    age = 28;
    println!("My Name is {} and I am {}", name, age);

    //Define Const
    //For const variable should be Uppercase and we need to explicity define variable type like i32 (Integer of 32 Bit)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign Multiple variables 
    let (my_name, my_age) = ("Asad", 24);
    println!("{} is {}", my_name, my_age);

}