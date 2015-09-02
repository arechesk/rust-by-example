Компилятор способен предоставить базовые реализации некоторых типажей посредством
[атрибута][attribute] `#[derive]`. Эти же типажи могут быть реализованы вручную,
если требуется более сложное поведение.

{derive.play}

Вот список выводимых компилятором типажей:
* Типажи сравнения:
  [`Eq`][eq],
  [`PartialEq`][partial-eq],
  [`Ord`][ord],
  [`PartialOrd`][partial-ord]
* [`Clone`][clone],
  для создания `T` из `&T` посредством копирования.
* [`Hash`][hash], для
  вычисления хеша из `&T`
* [`Default`][default],
  для создания пустого экземпляра типа данных.
* `Zero`, для
  создание нулевого экземпляра числового типа.
* [`Debug`][debug], для
  форматирования значения с помощью форматера `{:?}`.

[attribute]: /attribute.html
[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: http://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: http://doc.rust-lang.org/std/clone/trait.Clone.html
[hash]: http://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: http://doc.rust-lang.org/std/default/trait.Default.html
[debug]: http://doc.rust-lang.org/std/fmt/trait.Debug.html
