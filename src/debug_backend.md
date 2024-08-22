# Reporting backend crashes
If after a compilation failure you are greeted by a large amount of LLVM-IR code,
then our Enzyme backend likely failed to compile your code.
These cases are harder to debug, so your help is highly appreciated.
Please also keep in mind, that release builds are usually much more likely to work at the moment.

The final goal here is to reproduce your bug in the Enzyme [compiler explorer](https://enzyme.mit.edu/explorer/),
in order to create a bug report in the [Enzyme core](https://github.com/EnzymeAD/Enzyme/issues) repository.

We have an `autodiff` flag which you can pass to `RUSTFLAGS` to help with this. It will print the whole LLVM-IR module,
along with dummy functions called `enzyme_opt_dbg_helper_<i>`. A potential workflow on Linux could look like:  

## 1) Generate an LLVM-IR reproducer
```sh
RUSTFLAGS="-Z autodiff=OPT" cargo +enzyme build --release &> out.ll 
```
This also captures a few warnings and info messages above and below your module.
Open out.ll and remove every line above `; ModuleID = <SomeHash>`. 
Now look at the end of the file and remove everything that's not part of LLVM-IR, i.e. remove errors and warnings. 
The last line of your LLVM-IR should now start with `!<someNumber> = `, i.e.
`!40831 = !{i32 0, i32 1037508, i32 1037538, i32 1037559}` or `!43760 = !DILocation(line: 297, column: 5, scope: !43746)`.
The actual numbers will depend on your code.  

## 2) Check your LLVM-IR reproducer
To confirm that you're previous step worked, let's will use LLVM's opt tool. 
Find your path to the opt binary, with a path similar to 
`<some_dir>/rust/build/<x86/arm/...-target-tripple>/build/bin/opt`. 
Also find `LLVMEnzyme-19.<so/dll/dylib>` path, similar to `/rust/build/target-tripple/enzyme/build/Enzyme/LLVMEnzyme-19`. 
Once you have both, run the following command:
```sh
<path/to/opt out.ll> -load-pass-plugin=/path/to/LLVMEnzyme-19.so -passes="enzyme" -S 
```
If the previous step succeeded, you are going to see the same error that you saw when compiling your Rust code with Cargo. 
If you fail to get the same error, please open an issue in the Rust repository. If you succeed, congrats! 
The file is still huge, so let's automatically minimize it.

## 3) Minimize your LLVM-IR reproducer
First find your llvm-extract binary, it's in the same folder as your opt binary. Then run:
```sh
<path/to/llvm-extract> -S --func=<name> --recursive --rfunc="enzyme_opt_helper_*" out.ll -o mwe.ll 
```
Please adjust the name passed with the `--func` flag. 
You can either apply the `#[no_mangle]` attribute to the function you differentiate,
then you can replace it with the Rust name. Otherwise you will need to look up the mangled function name. 
To do that open out.ll and search for `__enzyme_fwddiff` or `__enzyme_autodiff`. 
The first string in that function call is the name of your function. Example:
```llvm-ir 
define double @enzyme_opt_helper_0(ptr %0, i64 %1, double %2) {
  %4 = call double (...) @__enzyme_fwddiff(ptr @_ZN2ad3_f217h3b3b1800bd39fde3E, metadata !"enzyme_const", ptr %0, metadata !"enzyme_const", i64 %1, metadata !"enzyme_dup", double %2, double %2)
  ret double %4
}
```
Here, `_ZN2ad3_f217h3b3b1800bd39fde3E` is the correct name. Make sure to not copy the leading `@`. 
Redo step 2), but now pass mwe.ll instead of out.ll to mod, to see if your minimized example reproduces your crash.

## 4) (Optional) Minimize your LLVM-IR reproducer further.
After the previous step you should have an `mwe.ll` file with ~5k LoC. Let's try to get it down to 50.
Find your `llvm-reduce` binary next to `opt` and `llvm-extract`. 
Copy the first line of your error message, an example could be:
```sh
opt: /home/manuel/prog/rust/src/llvm-project/llvm/lib/IR/Instructions.cpp:686: void llvm::CallInst::init(llvm::FunctionType*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::ArrayRef<llvm::OperandBundleDefT<llvm::Value*> >, const llvm::Twine&): Assertion `(Args.size() == FTy->getNumParams() || (FTy->isVarArg() && Args.size() > FTy->getNumParams())) && "Calling a function with bad signature!"' failed.
```
If you just get a segfault there is no sensible error message and not much to do automatically, so continue to 5).  
Otherwise, create a script.sh file containing
```sh
#!/bin/bash
<path/to/your/opt> $1 -load-pass-plugin=/path/to/LLVMEnzyme-19.so -passes="enzyme" \
    |& grep "/some/path.cpp:686: void llvm::CallInst::init"
```
Experiment a bit with which error message you pass to grep. It should be long enough to make sure that the error is unique. 
However, for longer errors including `(` or `)` you will need to escape them correctly which can become annoying. Run 
```sh 
<path/to/llvm-reduce> --test=script.sh mwe.ll 
```
If you see `Input isn't interesting! Verify interesting-ness test`, you got the error message in script.sh wrong, 
you need to make sure that grep matches your actuall error. 
If all works out, you will see a lot of iterations, ending with a new `reduced.ll` file. 
Verify with `opt` that you still get the same error.

## 5) Report your bug.

Afterwards, you should be able to copy and paste your `mwe.ll` (and `reduced.ll`) example into our [compiler explorer](https://enzyme.mit.edu/explorer/).  
Select `LLVM IR` as language and `opt 20` as compiler. Replace the field to the right of your compiler with `-passes="enzyme"`, if it is not already set. 
Hopefully, you will see once again your now familiar error. Please use the share button to copy links to them.

Please create an issue on [https://github.com/EnzymeAD/Enzyme/issues](github) and share `mwe.ll` and (if you have it) `reduced.ll`, as well as links to the compiler explorer. Please feel free to also add your Rust code or a link to it. With that, hopefully someone from the Enzyme core repository will be able to fix your bug. Once that happened, I will update the Enzyme submodule inside the rust compiler, which should allow you to now differentiate your Rust code. Thanks for helping us to improve Rust-AD.


# Minimize Rust code
Beyond having a minimal LLVM-IR reproducer, it is also helpful to have a minimal Rust reproducer without dependencies.
This allows us to add it as a testcase to CI once we fix it, which avoids regressions for the future.

There are a few solutions to help you with minimizing the Rust reproducer.
This is probably the most simple automated approach:
[cargo-minimize](https://github.com/Nilstrieb/cargo-minimize)

Otherwise we have various alternatives, including
[treereduce](https://github.com/langston-barrett/treereduce),
[halfempty](https://github.com/googleprojectzero/halfempty), or
[picireny](https://github.com/renatahodovan/picireny)

Potentially also
[creduce](https://github.com/csmith-project/creduce)

