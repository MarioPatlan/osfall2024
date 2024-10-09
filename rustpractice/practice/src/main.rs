fn main() {
    let nums = [1, 2, 3, 4, 5];

    // classic loop 
    for idx in 0..nums.len() { 
        // start inclusive ..end exclusive (it's fancy notation for range)
        println!("Element under idx {} := {}", idx, nums[idx]);
    }
}  
