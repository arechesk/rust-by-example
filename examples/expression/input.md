В Rust'е почти любой оператор является выражением:


```
fn main() {
    // statement
    // statement
    // statement
}
```

There are a few kinds of statements in Rust. The most common two are declaring
a variable binding, and using a `;` with an expression:
 что означает, что оператор возвращает значение. Это не всегда предпочтительно, 
 так что возвращаемые значения можно отбросить, добавив точку с запятой `;` к концу оператора.
```
fn main() {
    // связывание переменной
    let x = 5;

    // выражение;
    x;
    x + 1;
    15;
}
```

Блоки также являются выражениями, так что они могут быть использованы как [r-значения][rvalue] в операциях присваивания. Последнее выражение в блоке будет присвоено [l-значению][lvalue].
Однако, если последнее выражение в блоке окачивается точкой с запятой, как значение будет возвращено `()`

{expression.play}

[rvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
[lvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
