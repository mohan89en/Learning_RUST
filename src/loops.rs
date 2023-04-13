pub fn run(){
        let mut count = 0;

        //infinite loop
//         loop{
//           count +=1;
//           println!("Number: {}",count);
//           if count==21{
//                     break;
//           }
//         }  
        //while loop
        while count<=100{
          if count%15 ==0{
                    println!("Fizz Buzz");

          }
          else if count %3 ==0{
                    println!("Fizz");

          }
          else if count%5==0 {
              println!("Buzz");
          }
          else{
                    println!("{}",count);
          }
          count +=1;
        }
}
pub fn for_loop(){
     for x in 0..100{
          if x%15 ==0{
                    println!("Fizz Buzz");

          }
          else if x %3 ==0{
                    println!("Fizz");

          }
          else if x%5==0 {
              println!("Buzz");
          }
          else{
                    println!("{}",x);
          }  
     }     
}