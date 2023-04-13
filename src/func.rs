pub fn run(){
     greeting("hello","mohan");
     let get_sum = sum(1,2);
     println!("sum: {}",get_sum);
     //closure
     
     let add_num = |n1:i32,n2:i32|n1+n2;
     println!("sum: {}",add_num(1,4));
}

fn greeting(greet:&str,name:&str)
{
          println!("{} {}, nice meeting",greet,name);

}
fn sum(n1: i32,n2: i32) -> i32 {
n1+n2
}