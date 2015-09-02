fn main() {
    // Эта переменная живет в функции main
    let long_lived_variable = 1;

    // Это блок, он имеет меньшую облась видимости, чем функция main
    {
        // Эта переменная существует только в этом блоке
        let short_lived_variable = 2;

        println!("inner short: {}", short_lived_variable);

        // Эта переменная *скрывает* собой внешнуюю
        let long_lived_variable = 5_f32;

        println!("inner long: {}", long_lived_variable);
    }
    // Конец блока

    // Ошибка! `short_lived_variable` нет в этой области видимости
    println!("outer short: {}", short_lived_variable);
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    println!("outer long: {}", long_lived_variable);
}
