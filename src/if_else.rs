pub fn run(){
     let age:u8 = 17;
     if age>=18{
          println!("you are adult");
     } 
     else {
          println!("you are minor");
     }  
     //short-hand if
     let is_of_age = if age>=21{true} else{false};
     println!("age= {}", is_of_age); 
}