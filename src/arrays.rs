pub fn run(){
 let numbers: [i32; 5] = [1,2,3,4,5];
 println!("{:?}", numbers);
 
 // Get single vakue
 println!("Single value: {}", numbers[0]);  

 //Arrays are stack allocated
 println!("This array takes {} bytes in mem", std::mem::size_of_val(&numbers));

 //Get slice
 let slice: &[i32] = &numbers[1..3];
 println!("Slice: {:?}",slice);
}