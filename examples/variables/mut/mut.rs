fn main() {
    let _immutable_variable = 1;
    let mut mutable_variable = 1;

    println!("Перед изменением: {}", mutable_variable);

    // Ок
    mutable_variable += 1;

    println!("После изменения: {}", mutable_variable);

    // Ошибка!
    _immutable_variable += 1;
    // ИСПРАВЬТЕ ^ Закоментируйте эту строку
}
