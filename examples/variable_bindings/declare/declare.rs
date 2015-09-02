fn main() {
    // Объявляем переменную
    let a_variable;

    {
        let x = 2;

        // Инициализируем переменную
        a_variable = x * x;
    }

    println!("переменная: {}", a_variable);

    let another_variable;

    // Ошибка! Использование неинициализированной переменной
    println!("другая переменная: {}", another_variable);
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    another_variable = 1;

    println!("другая переменная: {}", another_variable);
}
