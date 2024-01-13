# itfs

Rust iterators and iterator adaptors useful when iterating over the file system.

-   **[component_filter]**: Export the struct **`ComponentFilter`**. Filter items where any of its
    path's [Components][Components] equals one given as parameter.
-   **[entry_to_path]**: Export the struct **`EntryToPath`**. Maps an iterator over items of type
    [`DirEntry`][DirEntry] or `Result<DirEntry>` into one over items of type [`PathBuf`][PathBuf]
    and `Result<PathBuf>` respectively.
-   **[only_extensions]**: Export the struct **`AllowExtensions`**. Only will let through entries
    which extensions are in a list of "allowed" ones.
-   **[path_reroot]**: Export the struct **`PathReRoot`**. Given an iterator over items of type
    [PathBuf] rewrite the root of those that contains a given prefix, by using another one given as
    a replacement.
-   **[rdr]**: Export the struct **`ReadDirRecursive`**. Iterator similar to the standard
    [`fs::ReadDir`][ReadDir] but recursive.
-   **[result_filter]**: Export the struct **`ResultFilter`**. It maps an iterator over items of
    type `Result<T>` into one over items of type `T` by discarding [`Err`][Err] variants.

## Extensions

There is also the [ext] module which expose other modules that implement traits that extends pre
existing types with new functionality / methods.

## Generate and open the documentation

```bash
cargo doc --lib --open
```

[component_filter]: ./src/component_filter.rs
[entry_to_path]: ./src/entry_to_path.rs
[ext]: ./src/ext.rs
[only_extensions]: ./src/only_extensions.rs
[path_reroot]: ./src/path_reroot.rs
[rdr]: ./src/rdr.rs
[result_filter]: ./src/result_filter.rs
[DirEntry]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
[PathBuf]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
[ReadDir]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
[Err]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
[components]: https://doc.rust-lang.org/stable/std/path/struct.Components.html
