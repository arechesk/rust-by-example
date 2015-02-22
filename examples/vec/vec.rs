fn main() {
    // Итераторы можно собрать в вектора
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // Для инициализации вектора можно использовать макрос `vec!`
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Вставляет новый элемент в конце вектора
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Ошибка! Неизменяемые вектора не могут увеличиваться
    collected_iterator.push(0);
    // ИСПРАВИТЬ ^ Закомментировать эту строку

    // Метод `len` возвращает текущий размер вектора
    println!("Vector size: {}", xs.len());

    // Обращение к элементам вектора записывается с помощью квадратных скобок
    // (нумерация элементов начинается с 0)
    println!("Second element: {}", xs[1]);

    // `pop` удаляет последний элемент из вектора и возвращает его
    println!("Pop last element: {:?}", xs.pop());

    // Обращение к элементу за пределами вектора вызывает ошибку
    println!("Fourth element: {}", xs[3]);
}
