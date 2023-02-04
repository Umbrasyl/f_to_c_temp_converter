use std::io;

fn main() {
    println!("Welcome to the temperature converter!");
    println!("Write 'f' if you would like to convert from fahrenheit, 'c' otherwise.");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line...");
    let mode = mode.trim();
    println!("Enter the temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line...");
    let temp: f32 = temp.trim().parse().expect("Failed to parse into float.");
    let result: f32;
    if mode == "c" {
        result = c_to_f(temp);
    } else {
        result = f_to_c(temp);
    }
    println!("The result is: {result}");
}

fn f_to_c(f: f32) -> f32 {
    let c = ((f - 32.0) * 5.0) / 9.0;
    return c;
}

fn c_to_f(c: f32) -> f32 {
    let f = ((c * 9.0) / 5.0) + 32.0;
    return f;
}
