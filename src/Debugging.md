# How to Debug AD?

Since Rust-AD is still in early development, crashes are not unlikely.

### Frontend crashes
The first case is that you cause a crash of our Rust frontend.
Luckily, these are trivial to debug. In most cases it will be enough
to remove the whole body of the function being differentiated 
(or consider replacing it with `loop {}`,
which matches almost every return type). 
Such a reproducer is trivial to fix, so please let us know!

For the unexpected case, that you produce an ICE in our frontend that 
is harder to minimize, please consider using [icemelter](https://github.com/langston-barrett/icemelter).

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

### Supported Environment Variables
To support you while debugging, we have added support for various environment variables,
which allow changing the behaviour of Enzyme, without recompiling rustc.
If you change your environment variables, you may need to run `cargo clean` to see the new behaviour.
We currently support:
```bash
export ENZYME_PRINT_TA=1
export ENZYME_PRINT_AA=1
export ENZYME_PRINT_PERF=1
export ENZYME_PRINT=1
export ENZYME_PRINT_MOD_BEFORE=1
export ENZYME_PRINT_MOD_AFTER_ENZYME=1
export ENZYME_PRINT_MOD_AFTER_OPTS=1
export ENZYME_LOOSE_TYPES=1
```

For performance experiments and benchmarking we also support
```bash
export ENZYME_NO_MOD_OPT_AFTER=1
export ENZYME_ENABLE_FNC_OPT=1
export ENZYME_NO_VEC_UNROLL=1
export ENZYME_NO_SAFETY_CHECKS=1
export ENZYME_INLINE=1
export ENZYME_ALT_PIPELINE=1
```
The normal compilation pipeline of Rust-Enzyme is
1) Run your selected compilation pipeline. If you selected a release build, we will disable vectorization and loop unrolling.
2) Differentiate your functions.
3) Run your selected compilation pipeline again on the whole module. This time we do not disable vectorization or loop unrolling.

The alt pipeline will not run opts before AD, but 2x after AD - the first time without vectorization or loop unrolling, the second time with.

The two flags above allow you to adjust this default behaviour.
