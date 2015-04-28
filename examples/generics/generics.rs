// Обобщённая структура
struct Pair<T> {
    first: T,
    second: T,
}

// Обобщённая функция
fn swap<T>(pair: Pair<T>) -> Pair<T> {
    let Pair { first, second } = pair;

    Pair { first: second, second: first }
}

// Реализовать кортеж из двух элементов как кортежеподобную структуру
struct Tuple2<T, U>(T, U);

fn main() {
    // Явно специализировать `Pair`
    let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };

    // Неявно специализировать `Pair`
    let pair_of_ints = Pair { first: 1i32, second: 2 };

    // Явно специализировать `Tuple2`
    let _tuple: Tuple2<char, i32> = Tuple2('R', 2);

    // Явно специализировать `swap`
    let _swapped_pair_of_chars = swap::<char>(pair_of_chars);

    // Неявно специализировать `swap`
    let _swapped_pair_of_ints = swap(pair_of_ints);
}
