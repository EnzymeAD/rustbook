# How to Debug AD?

Since Rust-AD is still in early development, crashes are not unlikely.

### Frontend crashes
The first case is that you cause a crash of our Rust frontend.
Luckily, these are trivial to debug. In most cases it will be enough
to remove the whole body of the function being differentiated 
(or consider replacing it with `loop {}`,
which matches almost every return type). 
Such a reproducer is trivial to fix, so please let us know!


### Backend crashes
Few lines of Rust code will expand into much larger LLVM-IR.
It is therefore beneficial to reduce a Rust reproducer as 
far as possible, before trying to minimize the generated LLVM-IR.
While manual minimization can not always be avoided, here are 
some tools, that might help.

This is probably the most simple automated approach:
[cargo-minimize](https://github.com/Nilstrieb/cargo-minimize)

Otherwise we have various alternatives, including
[treereduce](https://github.com/langston-barrett/treereduce),
[halfempty](https://github.com/googleprojectzero/halfempty), or
[picireny](https://github.com/renatahodovan/picireny)

Potentially also
[creduce](https://github.com/csmith-project/creduce)


