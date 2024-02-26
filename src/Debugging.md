# How to Debug AD?

Few lines of Rust code can expand into very large LLVM-IR.
It is therefore beneficial to reduce a Rust reproducer as 
far as possible, before trying to minimize the generated LLVM-IR.
While manual minimization can not always be avoided, here are 
some tools, that might help.

This is probably the most simple automated approach:
https://github.com/Nilstrieb/cargo-minimize

Otherwise we have various alternatives, including
https://github.com/langston-barrett/treereduce
https://github.com/googleprojectzero/halfempty
https://github.com/renatahodovan/picireny

potentially also
https://github.com/csmith-project/creduce


