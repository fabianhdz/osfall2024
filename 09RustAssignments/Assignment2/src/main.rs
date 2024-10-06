fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let arr: [i32; 10] = [1, 23, 34, 101, 45, 56, 67, 78, 89, 90];
    for num in arr.iter() {
        if is_even(*num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
        if num % 5 == 0 && num % 3 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }
    let mut array_size: i32 = arr.len() as i32;
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while array_size != 0 {
        sum += arr[i];
        array_size -= 1;
        i += 1;
    }
    println!("Sum of array: {}", sum);
    let mut i: usize = 0;
    let mut current_largest_number = arr[i];
    loop {
        i += 1;
        if i == arr.len() {
            break;
        }
        if arr[i] > current_largest_number {
            current_largest_number = arr[i];
        }
    }
    println!("The largest number is {}", current_largest_number);
}
