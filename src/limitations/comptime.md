# Compile Times

Enzyme will often achieve excellent runtime performance, but might increase your compile time by a large factor. 
For Rust, we already have made significant improvements and have a list of further improvements planed - please reach out if you have time to help here.

## Type Analysis
Most of the times, Type Analysis (TA) is the reason of large (>5x) compile time increases when using Enzyme. 
This poster explains why we need to run Type Analysis in the bottom left part: [Poster Link](https://c.wsmoses.com/posters/Enzyme-llvmdev.pdf).

Enzyme's TA will create TypeTrees based on usage patterns in the code.
Due to a suboptimal datastructure this process scales very poorly. 
Transfer the code (~1200 Lines of C++) to a better suited trie should remove most of this overhead, please reach out if you can help.
For the meantime, we do initialize TypeTrees for outermost function (those to which you apply '#[autodiff(...)]` based on the Rust types. 
In some real-worl applications (50k LoC), this improved the compile times by over 1000x - reducing them from hours to single minutes. 

## Duplicated Optimizations
The key reason for Enzyme offering often excellent performance is that Enzyme does differentiate already optimized LLVM-IR. 
However, we also (have to) run LLVM's optimization pipeline after differentiating, to make sure that the code which Enzyme generates is optimized properly. 
This is currently done approximately, but in certain cases some code will be optimized too often, while other code is not getting optimized enough. Tuning this could allow both compile time and runtime improvements.


## FAT-LTO 
The usage of '#[autodiff(...)]' currently requires compiling your project with fat-lto. 
We technically only need lto if the function being differentiated calls functions in other compilation units. 
Therefore other solutions are possible but this is the most simple one to get started. 
The compile time overhead of lto is small compared to the current compile time overhead of differentiating larger functions so this limitation is currently not a priority.

