
//       struct Color{
//           red:u8,
//           green:u8,
//           blue:u8,
//       }  

//tupple struct
struct Color(u8,u8,u8);
pub fn run(){
//           let mut c = Color{
//                     red:255,
//                     green:0,
//                     blue:0,
//           };
//           c.red = 200;
//      println!("color:{} {} {}",c.red,c.green,c.blue);
let mut c = Color(255,0,0);
c.1 = 200;
println!("{} {} {}",c.0,c.1,c.2);

}