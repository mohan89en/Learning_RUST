// varaibles hold primitive data or reference to data
// varaibles are immutable by default
// Rust is a block-scoped language
pub fn run()
{
          let name = "test";
          let mut age = 12;
          age = 13;
          println!("{} is {}", name, age);
          const ID:i32= 001;
          println!("ID: {}", ID);
          // assign multiple values
          let (my_name ,my_age)=("sitas",18);
          println!("{} is {}", my_name, my_age);
}