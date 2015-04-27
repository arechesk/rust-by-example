fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Ошибка изменяемости
    //*immutable_box = 4;

    // Передать коробку, заодно поменяв изменяемость
    let mut mutable_box = immutable_box;

    println!("mutable_box contained {}", mutable_box);

    // Изменить содержимое коробки
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
