fn main() {
    // `n` будет принимать значения 1, 2, ..., 100 с каждой итерацией
    for n in 1u32..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
