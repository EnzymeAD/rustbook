# Runtime Performance

While Enzymes performance should already be good in most cases, there are some optimizations left to apply. One is mentioned in the following compile time section.
The other optimization left to apply is re-enabling Rust's ABI optimizations.
The Rust compiler might change how Rust types are represented on a lower level, to allow faster function calls. These optimizations are mainly relevant when you call a small functions many times. 
We don't expect this to be the main application of autodiff, where we assume that you will often differentiate math-heavy code that for example calls faer, ndarray, or nalgebra matrix operations. 
We therefore disabled this optimization for the outermost function (the one to which one applies '#[autodiff(...)]`, to enable compile time improvements. 
However, it would be nice to teach Enzyme about these Rust ABI optimizations so we can have the best of both worlds.
