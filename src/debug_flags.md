# Supported RUSTFLAGS
To support you while debugging, we have added support for an experimental `-Z autodiff` flag to `RUSTFLAGS`,
which allow changing the behaviour of Enzyme, without recompiling rustc.
We currently support the following values for `autodiff`:

### Debug Flags
```bash
PrintTA // Print TypeAnalysis information
PrintAA // Print ActivityAnalysis information
Print // Print differentiated functions while they are being generated and optimized
PrintPerf // Print AD related Performance warnings
PrintModBefore // Print the whole LLVM-IR module before running opts
PrintModAfterOpts // Print the whole LLVM-IR module after running opts, before AD
PrintModAfterEnzyme // Print the whole LLVM-IR module after running opts and AD
LooseTypes // Risk incorect derivatives instead of aborting when missing Type Info 
OPT // Most Important debug helper: Print a Module that can run with llvm-opt + enzyme
```

<div class="warning">

`LooseTypes` is often helpful to get rid of Enzyme errors stating
`Can not deduce type of <X>` and to be able to run some code. But please 
keep in mind that this flag absolutely has the chance to cause incorrect gradients.
Even worse, the gradients might be correct for certain input values, but not for others.
So please create issues about such bugs and only use this flag temporarily while you wait for your 
bug to be fixed.

</div>

### Benchmark flags
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

You can combine multiple `autodiff` values using a comma as separator:
```bash
RUSTFLAGS="-Z autodiff=LooseTypes,NoVecUnroll" cargo +enzyme build
```


The normal compilation pipeline of Rust-Enzyme is
1) Run your selected compilation pipeline. If you selected a release build, we will disable vectorization and loop unrolling.
2) Differentiate your functions.
3) Run your selected compilation pipeline again on the whole module. This time we do not disable vectorization or loop unrolling.

The alt pipeline will not run opts before AD, but 2x after AD - the first time without vectorization or loop unrolling, the second time with.

The two flags above allow you to adjust this default behaviour.
