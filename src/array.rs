use std::mem;
pub fn print(){
    let numbers:[i32;5]=[1,2,3,4,5];
    println!("{:?}", numbers);  
    println!("{}",numbers[0]);   
    // arrays are stack allocated
    println!("{}",mem::size_of_val(&numbers));

    let slice: &[i32]= &numbers[0..2];
    println!("slice: {:?}",slice);
}
// vectors are resizable arrays
pub fn vector(){
          let mut numbers:Vec<i32>=vec![1,2,3,4,5];
          numbers.push(1);
    println!("{:?}", numbers);  
    println!("{}",numbers[0]);   
    // arrays are stack allocated
    println!("{}",mem::size_of_val(&numbers));
    

    let slice: &[i32]= &numbers[0..2];
    println!("slice: {:?}",slice);

    for x in numbers.iter(){
          println!("Number: {}",x);
    }
    // mutate values in a vector
    for x in numbers.iter_mut(){
          *x *= 2;
    }
    println!("{:?}",numbers)

}