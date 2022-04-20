pub fn run() {
    let name = "Thomas";
    let mut age = 19;
    println!("My name is {} and I am {}", name, age);
    age = 20;
    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Thomas", 100);
    println!("{} is {}", my_name, my_age)
}