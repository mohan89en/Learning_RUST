

enum Flowers{
     jasmine,
     rose,
     lily
}
struct Design{
          flower:Flowers,
          patterns_no:f64,
}
fn print_design(design:Design){
          match design.flower{
                    Flowers::jasmine => println!(" hey it is Jasmine"),
                    Flowers::rose => println!("rose"),
                    Flowers::lily=> println!("lily")
          }
          println!("no: {:?}",design.patterns_no);
}

pub fn run(){
     let sweet = Design{flower: Flowers::jasmine,
          patterns_no: 6.0};
     print_design(sweet);
      
     let new_design = Design{flower:Flowers::lily,patterns_no: 5.0};
     print_design(new_design)
     }

