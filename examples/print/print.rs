fn main() {
    // `print!`, как `println!`, но он не добавляет новую строку в конце
    print!("January has ");

    // `{}` это заполнители для аргументов, которые будут строками
    println!("{} days", 31i);
    // `i` суффикс указывает компилятору, что этот литерал имеет тип: целое
    // число со знаком, смотрите следующую главу для более подробной информации

    // Позиционные аргументы могут быть повторно использованы по шаблону
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Аргументы можно называть
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Специальное форматирование может быть указано в заполнителе после `:`, `t` это бинарное представление
    println!("{} of {:t} people know binary, the other half don't", 1i, 2i);

    // Ошибка! Не хватает аргумента для вывода
    println!("My name is {0}, {1} {0}", "Bond");
    // ИСПРАВЬТЕ ^ добавьте отсутствующий аргумент: "James"
}
