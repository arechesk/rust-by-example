fn main() {
    let captured_value = 7u32;

    let closure = |argument| {
        println!("Я захватило вот что: {}", captured_value);
        println!("Как аргумент было передано: {}", argument);

        true
    };

    println!("Замыкание вернуло: {}", closure("a string"));
}
