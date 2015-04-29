use std::marker::PhantomData;

// Обобщённая кортежная структура с одними и теми
// же типами в обобщении <A> и в определении (A,)
struct Tuple<A>(A,);

// Этот кортеж является фантомным типом, B — скрытый параметр.
// Место в памяти выделяется под А, но не под B. Поэтому B
// не может быть использован в вычислениях.
#[derive(PartialEq)] // Allow equality test for this type
struct PhantomTuple<A, B>(A,PhantomData<B>);

// Аналогично, фантомная структура обобщённая над А со скрытым
// параметром B
#[derive(PartialEq)] // Allow equality test for this type
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

fn main() {
    // instantiate Tuple кортеж (для сравнения)
    let _tuple: Tuple<char> = Tuple('R');

    // We can create similar types without carrying around extra info
    // PhantomTuple specialized to <char, f32>
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple specialized to <char, f64>
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Error: type mismatch so these cannot be compared
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);

    // Specialized to <char, f32>
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Specialized to <char, f64>
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Error: type mismatch so these cannot be compared
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
}

