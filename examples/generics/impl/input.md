Реализации также могут быть обобщены. Вообще говоря, за `impl` следует тип
`<Type>`, хотя это не строгое требование.

```rust
struct S; // Пустая структура
struct GenericTup<T>(T,);

// специализируем GenericTup для конкретных типов:
impl GenericTup<f32> {} // Специализируем для `f32`
impl GenericTup<S> {} // Специализируем для `S`, определённого выше

// `<T>` должно быть перед типом, чтобы он оставался обобщённым
impl <T> GenericTup<T> {}
```

{impl.play}

###См. также:

[impl](http://rustbyexample.com/methods.html),
[struct](http://rustbyexample.com/structs.html), and
[functions returning references](http://rustbyexample.com/lifetime/fn.html),

