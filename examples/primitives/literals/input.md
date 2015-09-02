## Литералы и операторы

Целые `1`, числа с плавающей точкой `1.2`, символы `'a'`, строки `"abc"`, логический `true` и единичный типы `()` могут быть выражены как литералы.

Целые числа можно также выразить с помощью шестнадцатеричной, восьмеричной или двоичной записи, используя, соотвестственно, префиксы `0x`, 0o` или `0b`.

В числовые литералы можно вставлять подчёркивания для читабельности, например: `1_000` то же самое, что `1000`, а `0.000_001` то же самое, что `0.000001`.

Нам требуется сказать компилятору, литералы какого типа мы используем. Пока что мы будем использовать суффикс `u32`, чтобы указать, что литерал является беззнаковым 32-битовым целым, и суффикс `i32`, для знаковых 32-битовых целых. Мы рассмотрим систему типов в [другой главе][type], а детали, касающиеся литералов — в [соотвествующем параграфе][type-literal].

Имеющиеся операторы и их приоритет схожи с другими [C-подобных языками][op-prec].

{literals.play}

[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
[type]: /type.html
[type-literal]: /type/literals.html