pub fn run(){
          //primitive array
          let arr1 = [1,2,3];
          let arr2 = arr1;
          println!("values: {:?}", (arr1,arr2));
          // with non primitive if you assign varaiables to a piece of data,
          //the first varaible will no longer hold the value and you need to use refernce to point to resource
          let vec1 = vec![1,2,3];
          let vec2 = &vec1;
          println!("values: {:?}", (&vec1,vec2));
}