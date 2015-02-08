fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Это выражение будет присвоенно переменной `y`
        x_cube + x_squared + x
    };

    let z = {
        // Точка с запятой отбрасывает это выражение, и `z` будет присвоено значение `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
