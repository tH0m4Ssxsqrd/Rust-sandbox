pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age:bool = false;
    let knows_person_is_minor:bool = true;

    if age >= 18 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 18 && check_id || knows_person_is_minor{
        println!("Sorry, you have to leave");
    } else {
        println!("I'll need to see your ID")
    }
}