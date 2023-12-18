# rustc Design:
This chapter is not relevant for an autodiff user, but might still be interesting for thos curious to include Enzyme into a new language. It is mostly tailored towards people already working on rustc.

The first step was to integrate Enzyme (core) itself. We have added it as a submodule to `src/tools/enzyme` and updated the bootstraping accordingly. 
We had added our autodiff macro in `/library`, but this approach had to be abandonned for cross-compilation). The current macro is implemented as a `rustc_internal_macro`.

There do have to alter the compilation pipeline in a few aspects when autodiff is used.
The obvious one is that we have to prevent our source function from getting competely inlined. We do this in <> by marking it as <>

