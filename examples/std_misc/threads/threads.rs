use std::thread;

static NTHREADS: i32 = 10;

// Это основной поток `main`
fn main() {
    for i in 0..NTHREADS {
        // Запустить еще один поток
        let _ = thread::scoped(move || {
            println!("this is thread number {}", i)
        });
    }
}
