pub fn run(){
          let mut hello = String::from("hello world ");
          println!("{}", hello.len());//length of the string
          hello.push('W');
          hello.push_str(" I am fine");
          println!("{}-{}", hello,hello.capacity());
          //.is_empty() = to see if its empty
          //.contains to check the presence of the substitution
          //.replace to replace a string
           let mut s = String::with_capacity(10);
           s.push('I');
           s.push_str(" am");
           //assertion testing(to check the equality)
           assert_eq!(10,s.len());
           println!("{}",s);
}