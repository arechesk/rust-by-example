Реализации также могут быть обобщены. Вообще говоря, за `impl` следует тип
`<Type>`, хотя это не строгое требование.

```rust
struct S; // Пустая структура
struct GenericVal<T>(T,);

// специализируем GenericVal для конкретных типов:
impl GenericVal<f32> {} // Специализируем для `f32`
impl GenericVal<S> {} // Специализируем для `S`, определённого выше

// `<T>` должно быть перед типом, чтобы он оставался обобщённым
impl <T> GenericVal<T> {}
```

{impl.play}

###См. также:

[functions returning references][fn], [`impl`][methods], and [`struct`][structs]


[fn]: /scope/lifetime/fn.html
[methods]: /fn/methods.html
[specialization_plans]: http://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: /custom_types/structs.html
