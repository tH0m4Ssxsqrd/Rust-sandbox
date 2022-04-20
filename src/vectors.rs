//Vectors are resizable arrays

pub fn run(){
    let numbers: Vec<i32> = vec![1,2,3,4,5];
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
   }