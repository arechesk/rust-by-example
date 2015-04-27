// Эта функция берёт во владение память, выделеную в куче
fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);

    // `c` будет уничтожено в этой области видимости, а память освобождена
}

fn main() {
    // Выделенное в стеке целое
    let x = 5u32;

    // «Копировать» `x` в `y`, нет ресурсов для перемещения
    let y = x;

    // Оба значения можно использовать независимо
    println!("x is {}, and y is {}", x, y);

    // `a` — указатель на выделенное в куче целое
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // «Переместить» `a` в `b`
    // Вот как это работает: указатель `a` копируется (но *не данные* в куче,
    // только адрес) в `b`. Теперь они оба указывают на *одни и те же* данные
    // в куче. Однако теперь `b` *владеет* выделенными в куче данными, и именно
    // переменная `b` ответственна за освобождение этих данных.
    let b = a;

    // После предыдущего перемещения использовать `a` нельзя
    // Ошибка! `a` теперь не имеет доступ к данным, так как она больше не владеет
    // памятью в куче
    //println!("a contains: {}", a);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // «Переместить» `b` в функцию, `b` передаёт ей владение данными в куче
    destroy_box(b);

    // Так как память в куче в этой точке уже освобождена, данное действие
    // приведёт к разименованию освобожлённой памяти. К счастью, это запрещено
    // компилятором.
    // Ошибка! Причём та же, что и ранее.
    //println!("b contains: {}", b);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку
}
