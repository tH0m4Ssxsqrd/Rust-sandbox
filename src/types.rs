pub fn run(){
    // by default i32
    let x = 1;
    //by default f64
    let y = 2.5;
    //Add explicit
    let z: i64 = 45454545;

    //find max size
    println!("Max i32: {}", std::i128::MAX);

    let face: char = '\u{1F600}';

    println!("{:?}", (x, y, z, face));

}