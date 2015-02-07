fn main() {
    // `print!` подобен `println!`, но не добавляет символ новой строки в конце
    print!("January has ");

    // `{}` это заполнители для аргументов, которые будут превращены в строкам
    println!("{} days", 31);

    // Позиционные аргументы могут быть повторно использованы по шаблону
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // У аргументов могут быть имена
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Специальное форматирование может быть указано в заполнителе после `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // Ошибка! Пропущен агрумент
    println!("My name is {0}, {1} {0}", "Bond");
    // ИСПРАВЬТЕ ^ добавьте отсутствующий аргумент: "James"
}
