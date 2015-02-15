To link a crate to this new library, the `extern crate` declaration must be
used. This will not only link the library, but also import all its items
under a module named the same as the library. The visibility rules that apply
to modules also apply to libraries.
Чтобы связать крэйт с новой библиотекой, нужна декларация `extern crate`. Она не только свяжет библиотеку, но и импортирует все элементы в модуль с тем же именем, что и сама библиотека. Правила видимости, применимые к модулям, так же применимы и к библиотекам.

{executable.rs}

```
# Аргумент `-L .` добавляет текущию директорию к путям поиска библиотек
$ rustc -L . executable.rs && ./executable
вызвана `public_function()` крэйта erty
вызвана `indirect_access()` крэйта erty, которая
> вызывает `private_function()` крэйта erty
```
