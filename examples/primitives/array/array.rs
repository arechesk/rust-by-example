use std::mem;

// Эта функция берёт на время срез
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Массив заданного размера (объявление типа избыточно)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Можно инициализировать все элементы одним значением
    let ys: [i32; 500] = [0; 500];

    // Индексы в массиве начинаются с 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` возвращает размер массива
    println!("array size: {}", xs.len());

    // Массивы размещаются в стеке
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Массивы могут быть автоматически заимствованы как срезы
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Срезы могут указывать на часть массива
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Выход за пределы массива вызывает ошибку
    println!("{}", xs[5]);
}
