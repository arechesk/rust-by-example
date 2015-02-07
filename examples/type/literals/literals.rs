fn main() {
    // Литералы с суффиксами, их тип известен при инициализации
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Литералы без суффиксов, их тип зависит от того, как их используют
    let i = 1;
    let f = 1.0;

    // `size_of_val` возвращает размер переменной в байтах
    println!("размер `x` в байтах: {}", std::mem::size_of_val(&x));
    println!("размер `y` в байтах: {}", std::mem::size_of_val(&y));
    println!("размер `z` в байтах: {}", std::mem::size_of_val(&z));
    println!("размер `i` в байтах: {}", std::mem::size_of_val(&i));
    println!("размер `f` в байтах: {}", std::mem::size_of_val(&f));

    // Ограничения (слагаемые должны иметь тот же тип) для `i` и `f`
    let _constraint_i = x + i;
    let _constraint_f = z + f;
    // ЗАДАНИЕ ^ Закомментируйте эти две строки
}
