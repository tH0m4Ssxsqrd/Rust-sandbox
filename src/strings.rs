pub fn run(){
    /* Forever static string
    let hello = "Hello"; */
    let mut hello = String::from("Hello");
    
    // get lenght
    println!("Lenght: {}", hello.len());

    hello.push_str(" World. Today I would like to introduce you to rust.");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Loop through string by white space
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    //Create string with given capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


    println!("{}", hello);
}