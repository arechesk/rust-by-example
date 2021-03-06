Переменные в Rust не только держат данные в стеке, они также могут *владеть*
ресурсами; к примеру, `Box<T>` владеет памятью в куче. Поскольку Rust строго
придерживается [RAII][raii], то когда объект выходит за зону видимости, вызывается
его деструктор, а *владеемый* им ресурс освобождается. Такое поведение защищает
от багов, связанных с *утечкой ресурсов.*

{raii.play}

Незачем верить на слово, давайте проверим `valgrind`'ом:

```
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

Вам больше никогда не потребуется вручную освобождать память или же беспокояться об её утечках!

[raii]: http://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization
