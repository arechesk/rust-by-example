Rust не предоставляет неявного преобразования типов (coercion) между примитивными типаму, но явное приведение типов (casting) можно осуществить с помощью ключевого слова `as`.

Rules for converting between integral types follow C conventions generally,
except in cases where C has undefined behavior. The behavior of all casts
between integral types is well defined in Rust.
{cast.play}
