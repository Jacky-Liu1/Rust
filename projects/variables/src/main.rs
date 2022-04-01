fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = fahrenheit_to_celsius(32.0);
    println!("The value of y is: {}", y);


    let a = nth_fibonacci(10);
    println!("The value of a is: {}", a);
}


fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/9.0)
}

fn nth_fibonacci(num: i64) -> i64 {
    if num <= 2  {
        return 1;
    }
    return nth_fibonacci(num-1) + nth_fibonacci(num-2);
}