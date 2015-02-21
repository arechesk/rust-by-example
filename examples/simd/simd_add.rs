#![feature(test)]
#![feature(core)]

use std::simd::f32x4;

macro_rules! assert_equal_len {
    ($a:ident, $b: ident) => {
        assert!($a.len() == $b.len(),
                "add_assign: dimension mismatch: {:?} += {:?}",
                ($a.len(),),
                ($b.len(),));
    }
}

// поэлементное сложение
fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x += *y;
    }
}

// ускоренное сложение с помощью simd
fn simd_add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    let size = xs.len() as isize;
    let chunks = size / 4;

    // указатель на начало данных вектора
    let p_x: *mut f32 = xs.as_mut_ptr();
    let p_y: *const f32 = ys.as_ptr();

    // складываем лишние элементы, которые не поместились в вектор simd
    for i in (4 * chunks)..size {
        // для разыменования обычного указателя необходим блок unsafe
        unsafe {
            // сдвиг на i элементов
            *p_x.offset(i) += *p_y.offset(i);
        }
    }

    // считаем вектор f32 вектором simd f32x4
    let simd_p_x = p_x as *mut f32x4;
    let simd_p_y = p_y as *const f32x4;

    // складываем "вектора simd"
    for i in 0..chunks {
        unsafe {
            *simd_p_x.offset(i) += *simd_p_y.offset(i);
        }
    }
}

mod bench {
    extern crate test;
    use self::test::Bencher;
    use std::iter;
    static BENCH_SIZE: usize = 10_000;

    macro_rules! bench {
        ($name:ident, $func:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut x: Vec<_> = iter::repeat(1.0f32)
                                        .take(BENCH_SIZE)
                                        .collect();
                let y: Vec<_> = iter::repeat(1.0f32)
                                        .take(BENCH_SIZE)
                                        .collect();

                b.iter(|| {
                    super::$func(&mut x, &y);
                })
            }
        }
    }

    bench!(vanilla, add_assign);
    bench!(simd, simd_add_assign);
}
