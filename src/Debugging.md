# How to Debug AD?

Since Rust-AD is still in early development, crashes are not unlikely.

### Frontend crashes
If you see a proper Rust stacktrace after a compilation failure, our frontend (thus rustc) has likely crashed.
It should often be trivial to create a minimal reproducer, by deleting most of the body of the 
function being differentiated, or by replacing the function body with a `loop {}` statement.
Please create an issue with such a reproducer, it will likely be easy to fix!

For the unexpected case, that you produce an ICE in our frontend that 
is harder to minimize, please consider using [icemelter](https://github.com/langston-barrett/icemelter).

### Backend crashes
If after a compilation failure you are greeted by a large amount of LLVM-IR code,
then our Enzyme backend likely failed to compile your code.
These cases are harder to debug, so your help is highly appreciated.
Please also keep in mind, that release builds are usually much more likely to work at the moment.

The final goal here is to reproduce your bug in the Enzyme [compiler explorer](https://enzyme.mit.edu/explorer/),
in order to create a bug report in the [Enzyme core](https://github.com/EnzymeAD/Enzyme/issues) repository.

We have an environment variable called `ENZYME_OPT` to help with this. It will print the whole LLVM-IR module,
along with dummy functions called `enzyme_opt_dbg_helper_<i>`. A potential workflow on Linux could look like:  

`cargo clean && ENZYME_OPT=1 cargo +enzyme build &> out.ll`  
This also captures a few warnings and info messages above and below your module.
Open out.ll and remove every line above `; ModuleID = <SomeHash>` and every line below the last DILocation,
e.g. below `!43760 = !DILocation(line: 297, column: 5, scope: !43746)`. The actual numbers will depend on your code.  

`llvm-extract -S --func=f --recursive --rfunc="enzyme_opt_helper_*" out.ll -o mwe.ll`
Please also adjust the name passed with the `--func` flag if your function isn't called `f`. Either look up the correct
llvm-ir name for your function in out.ll, or use the `#[no_mangle]` attribute on the function which you differentiate, in which case 
you can pass the original Rust function name to this flag.

Afterwards, you should be able to copy and paste your mwe example into our [compiler explorer](https://enzyme.mit.edu/explorer/) and 
hopefully reproduce the same Enzyme error, which you got when you tried to compile your original Rust code.
Please select `LLVM IR` as a language and `opt 17` as your compiler and replace the LLVM-IR example with your final mwe.ll content.

You will quickly note that even small Rust function can generate large llvm-ir reproducer. Please try to get your llvm-ir function below
100 lines, by reducing the Rust function to be differentiated as far as possible. This will significantly speed up the bug fixing process.
Please also try to post both, the compiler-explorer link with your llvm-ir reproducer, as well as a self-contained Rust reproducer.

There are a few solutions to help you with minimizing the Rust reproducer.
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
We currently support the following `RUSTFLAG` values for autodiff:
```bash
PrintTA // Print TypeAnalysis information
PrintAA // Print ActivityAnalysis information
PrintPerf // Print AD related Performance warnings
Print // Print all of the above
PrintModBefore // Print the whole LLVM-IR module before running opts
PrintModAfterOpts // Print the whole LLVM-IR module after running opts, before AD
PrintModAfterEnzyme // Print the whole LLVM-IR module after running opts and AD
LooseTypes // Risk incorect derivatives instead of aborting when missing Type Info 
OPT // Most Important debug helper: Print a Module that can run with llvm-opt + enzyme
```

For performance experiments and benchmarking we also support
```
NoModOptAfter // We won't optimize the whole LLVM-IR Module after AD
EnableFncOpt // We will optimize each derivative function generated individually
NoVecUnroll // Disables vectorization and loop unrolling
NoSafetyChecks // Disables Enzyme specific safety checks
RuntimeActivity // Enables the runtime activity feature from Enzyme 
Inline // Instructs Enzyme to apply additional inlining beyond LLVM's default
AltPipeline // Don't optimize IR before AD, but optimize the whole module twice after AD
```

You can combine multiple `RUSTFLAG` values using a comma as separator:
```bash
RUSTFLAGS="-Z autodiff=LooseTypes,NoVecUnroll" cargo +enzyme build
```


The normal compilation pipeline of Rust-Enzyme is
1) Run your selected compilation pipeline. If you selected a release build, we will disable vectorization and loop unrolling.
2) Differentiate your functions.
3) Run your selected compilation pipeline again on the whole module. This time we do not disable vectorization or loop unrolling.

The alt pipeline will not run opts before AD, but 2x after AD - the first time without vectorization or loop unrolling, the second time with.

The two flags above allow you to adjust this default behaviour.
