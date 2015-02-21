#![feature(core)]

use std::simd::f32x4;

fn main() {
    // создаём вектора simd
    let x = f32x4(1.0, 2.0, 3.0, 4.0);
    let y = f32x4(4.0, 3.0, 2.0, 1.0);

    // умножаем при помощи simd
    let z = x * y;

    // как любую другую структуру, вектор simd можно разложить с помощью `let`
    let f32x4(a, b, c, d) = z;

    println!("{:?}", (a, b, c, d));
}
