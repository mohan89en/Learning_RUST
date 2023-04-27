pub fn run(){
    let num_terms = 10;
    
    // Initialize the first two terms of the sequence
    let mut prev = 0;
    let mut curr = 1;
    
    // Loop through each term of the sequence and print it
    for i in 0..num_terms {
        println!("Term {}: {}", i+1, prev);
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
}