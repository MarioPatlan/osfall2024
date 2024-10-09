fn is_even(n: i32) -> bool {

    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }

}

fn main() {

    let arr = [1, 2, 5, 7, 9, 12, 14, 15, 18, 22];

    for num in arr.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Buzz");
        } else if num % 5 == 0 {
            println!("Fizz");
        } else {
            if is_even(*num) == true {
                println!("even");
            } else {
                println!("odd");
            }
        }
    }

    let mut index_1: usize = 0;
    let mut arr_sum = 0;

    while index_1 < arr.len() {
        arr_sum = arr_sum + arr[index_1];
        index_1 += 1;
    };

    println!("The sum of the array is equal to {}", arr_sum);

    let mut index_2: usize = 0;
    let mut arr_largest = 0;

    loop {
        if index_2 == arr.len() {
            break;
        } else if arr_largest < arr[index_2] {
            arr_largest = arr[index_2];
            index_2 += 1;
        } else {
            index_2 += 1;
        } 
    }

    println!("The largest number in the array is {}", arr_largest);

    }