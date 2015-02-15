A crate is a compilation unit in Rust. Whenever `rustc some_file.rs` is called,
`some_file.rs` is treated as the *crate file*. If `some_file.rs` has `mod`
declarations in it, then the contents of the module files will get merged with
the crate file *before* running the compiler over it. In other words, modules
do *not* get compiled individually, only crates get compiled.
Крэйт — единица компиляции в Rust'е. Когда вызывается `rustc some_file.rs`, `some_file.rs` рассматривается как *файл крэйта*. Если в `some_file.rs` есть декларация `mod`, то содержимое модуля будет объединено с файлом крэйта *перед* его компиляцией. Другими словами, модули *не* собираются отдельно, собираются лишь крэйты.

A crate can be compiled into a binary or into a library. By default, `rustc`
will produce a binary from a crate. This behavior can be overridden by passing
the `--crate-type` flag to `rustc`.
Крэйт может быть скомпилирован в исполняемый файл или в библиотеку. По умолчанию, `rustc` создаёт исполняемый файл из крэйта. Это поведение может быть изменено добавлением флага `--crate-type` к `rustc`.
