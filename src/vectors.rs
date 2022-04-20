//Vectors are resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    // Reasign value
    numbers[2] = 20;
    
    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();
    
    println!("{:?}", numbers);

    // Get single vakue
    println!("Single value: {}", numbers[0]);
    
    // Get vector lenght
    println!("Vector lenght: {}", numbers.len());
   
    //Vectors are stack allocated
    println!("This vector takes {} bytes in mem", std::mem::size_of_val(&numbers));
   
    //Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("{}",x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
   }