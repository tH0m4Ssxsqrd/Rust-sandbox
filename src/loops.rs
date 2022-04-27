// Iterate until a condition is met

pub fn run(){
    let mut count = 1;
    
    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
    // }

    while count<=100{
        if count % 15 == 0 { println!("FizzBuzz")}
        else if count % 5 == 0 {println!("Buzz")}
        else if count % 3 == 0 {println!("Fizz")}
        else {println!("{}", count)}
        //increment
        count += 1
    }





}
