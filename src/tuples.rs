pub fn run(){
    let person: (&str, &str, i8) = ("Thomas", "Uni", 100);
    println!("{} is from {} and he is {}", person.0, person.1,person.2);
}