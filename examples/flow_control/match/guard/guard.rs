fn main() {
    let pair = (2i32, -2);
    // ЗАДАНИЕ ^ Попробуйте разные значения `pair`

    println!("Расскажи мне о {:?}", pair);
    // Сопоставление может разобрать значение
    match pair {
        // Деконструировать кортеж
        (x, y) if x == y => println!("Близнецы"),
        // Часть ^ `if ‹условие›` — охранное выражение
        (x, y) if x + y == 0 => println!("Антиматерия, бабах!"),
        // `_` означает не связывать значение с переменной
        (x, _) if x % 2 == 1 => println!("Первое число чётно"),
        _ => println!("Нет корреляции..."),
    }
}
