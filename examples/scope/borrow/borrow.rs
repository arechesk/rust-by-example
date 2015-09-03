// Эта функция берёт во владение коробку
fn eat_box(boxed_int: Box<i32>) {
    println!("destroying box that contains {}", boxed_int);
}

// А эта функция вместо этого заимствует i32
fn peep_inside_box(borrowed_int: &i32) {
    println!("This int is: {}", borrowed_int);
}

fn main() {
    // Упакованное целое
    let boxed_int = Box::new(5);

    // Заимствовать содержимое коробки, владение не передаётся
    peep_inside_box(&boxed_int);

    // Содержимое можно занять ещё раз
    peep_inside_box(&boxed_int);

    {
        // Получить ссылку на данные внутри коробки
        let _ref_to_int: &i32 = &boxed_int;

        // Ошибка! Нельзя уничтожить boxed_int, пока значение внутри взято
        // взаймы
        eat_box(boxed_int);
        // ИСПРАВЬТЕ ^ Закомментируйте эту строку

        // `_ref_to_int` покидает облать видимости
    }

    // Перестать владеть коробкой
    eat_box(boxed_int);
}
