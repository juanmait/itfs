# itfs

Rust iterator adaptors useful when iterating over the file system.

-   **entry_to_path**: Export the struct `EntryToPath`. Maps an iterator over items of type
    [`DirEntry`][DirEntry] or [`Result<DirEntry>`] into one over items of type [`PathBuf`][PathBuf]
    and [`Result<PathBuf>`] respectively.
-   **extension_filter**: Export the struct `ExtensionFilter`. Filter [`DirEntry`][DirEntry] items
    where the file extension is not in a list of allowed ones.
-   **rdr**: Export the struct `ReadDirRecursive`. Iterator similar to the standard
    [`fs::ReadDir`][ReadDir] but recursive.
-   **result_filter**: Export the struct `ResultFilter`. It maps an iterator over items of type
    `Result<T>` into one over items of type `T` by discarding [`Err`][Err] variants.

## Generate and open the documentation

```bash
cargo doc --lib --open
```

[DirEntry]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
[PathBuf]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
[ReadDir]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
[Err]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
