# ReadDirRecursive

A simple Rust library that creates an iterator similar to the standard
[fs::ReadDir](https://doc.rust-lang.org/std/fs/struct.ReadDir.html) but recursive.

**WARNING**: this is just a learning exercise. Please don't use it for anything. The recursive
iteration seems to be pretty fast but applied to directories with many files and/or subdirectories
can lead to panics due to stack overflows 💥. Seems like not much of the hip is being used and
everything goes into the stack 🔥. I will explore options to solve this later.
