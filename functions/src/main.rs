fn main() {
    println!("Hello, world!");
    print_measurement(10, 'm');
}

fn print_measurement(number: i32, unit_label: char) {
    println!("The measurement is: {number}{unit_label}");
    let sum = plus_one(5);
    println!("The result of the sum is {sum}")
}

fn plus_one(x: i32) -> i32 {
    x + 1
}