use std::env;

pub fn run(){
          let args: Vec<String> = env::args().collect();
          println!("Agrs:{:?}",args);
          let command = args[1].clone();
          println!("command: {:?}",command);
}