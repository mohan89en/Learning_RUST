enum Movement {
          //varaibles
    Up, Down, Left, Right
}
fn move_avatar(m:Movement){
  //perfom action
  match m{
          Movement::Up => println!("moving up"),
          Movement::Down => println!("moving Down"),
          Movement::Right => println!("moving Right"),
          Movement::Left => println!("moving Left"),
  }
}

pub fn print(){

let avatar1 = Movement::Left;
move_avatar(avatar1);

}