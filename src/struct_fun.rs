pub fn run(){

    struct Person{
          first_name: String,
          last_name: String
    }
    impl Person{
          fn new(first:&str,last:&str) ->Person{
                    Person{
                              first_name:first.to_string(),
                              last_name:last.to_string(),
                    }
          }
      fn full_name(&self) ->String { format!("{}{}", self.first_name,self.last_name) }
    }

    let mut p = Person::new("John","Doe");
    println!("{}{}",p.first_name,p.last_name);
    println!("{}",p.full_name())

}