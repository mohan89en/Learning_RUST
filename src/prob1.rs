fn solution1(intensity: i32, random_number: i32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay
hydrated!"
            );
        } else {
            println!("Today, run for {} minutes!", expensive_result)
        }
    }
}
