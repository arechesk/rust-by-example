An attribute is metadata applied to some module, crate or item. This metadata
can be used to/for:
Атрибуты это метаданные, применяемые к какому-либо модулю, крэйту или их элементу. Эти метаданные используются, чтобы:

<!-- TODO: Link these to their respective examples -->
* [задать условия компиляции кода][cfg]
* [задать имя, версию и тип (библиотека или исполняемый файл) крэйта][crate]
* отключить [lints][lint] (предупреждения)
* включить возможностей компилятора
* связаться с внешней библиотекой
* пометить функций как юнит тесты
* пометить функции, которые будут частью бенчмарка

When attributes apply to a whole crate, their syntax is `#![crate_attribute]`,
and when they apply to a module or item, the syntax is `#[item_attribute]`
(notice the missing bang `!`).
Когда атрибуты применяются ко всему крэйту, их синтаксис `#![crate_attribute]`, когда же они применяются к модулю или элементу модуля, их синтаксис `#[item_attribute]` (обратите внимание на отсутствие `!`).

Attributes can take arguments with different syntaxes:
Атрибуты могут принимать аргументы с различным синтаксисом:

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

[cfg]: /attribute/cfg.html
[crate]: /attribute/crate.html
[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
