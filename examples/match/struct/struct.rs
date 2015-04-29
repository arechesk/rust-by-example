fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // деконструировать поля структуры
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // можно деконструировать структуры и переименовывать переменные,
    // порядок не важен

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // ещё можно опустить некоторые переменные:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // это вызовет ошибку: в образеце не упомянуто поле `x`
    // let Foo { y } = foo;
}
