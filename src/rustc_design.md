# rustc Design:
This chapter is not relevant for an autodiff user, but might still be interesting for those curious to include Enzyme into a new language. It is mostly tailored towards people already working on rustc. I would like to claim it is also there to help reviewers, but realistically I just write things down here because I can't remember what I coded yesterday. This is likely incomplete, so please create PR's to extend it! 

The first step was to integrate Enzyme (core) itself. We have added it as a submodule to `src/tools/enzyme` and updated the bootstrapping accordingly. 
We had added our autodiff macro in `/library`, but this approach had to be abandoned for cross-compilation. The current macro is implemented as a `rustc_internal_macro`.

We had to alter the compilation pipeline in a few places when autodiff is used.
The obvious one is that we have to prevent our source function from getting completely inlined. 

`compiler/rustc_ast/src/expand/autodiff_attrs.rs`
This is a new file, containing the logic to parse our autodiff macro into rustc 
builtin `rustc_autodiff` attributes. 
This blocks users from tinkering with our internals, 
by having to go through the macro.
It has the nice side-effect, that we can implement an erroring dummy fallback if 
we see that the `llvm_enzyme` config hasn't been set when building rustc.

`compiler/rustc_codegen_llvm/src/attributes.rs`: 
In `from_fn_attrs` we query `cx.tcx.autodiff_attrs(instance.def_id());`
and if we get an autodiff is active (that is a source or a placeholder). 
This is to make sure that neither of the too gets inlined, for that we mark them as `InlineAttr::Never` and pray for the best.

`compiler/rustc_codegen_ssa/src/codegen_attrs.rs`
We added `autodiff_attrs`, in which we parse `rustc_autodiff` attributes applied to Functions, and create `AutoDiffAttrs` out of it, which we return.

`compiler/rustc_monomorphize/src/partitioning.rs`
In `place_mono_items` we check if the `characteristic_def_id` of a function exists, and if yes (and if it has an autodiff attrs) we block it from being inserted into `internalization_candidates` to prevent inlining.

`compiler/rustc_monomorphize/src/partitioning.rs`
In `collect_and_partition_mono_items` we update things.

`compiler/rustc_codegen_ssa/src/back/write.rs`
In `generate_lto_work` we pass the autodiff worklist to our backend autodiff function doing the actual work. We check there that our autodiff worklist is empty if we don't use fat-lto.


`compiler/rustc_middle/src/ty/mod.rs`
How to create a typetree or fnctree from a `rustc_middle::ty`
