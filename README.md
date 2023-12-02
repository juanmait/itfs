# fs_iter

Some rust iterator adaptors useful when iterating over the file system.

-   **extension_filter**: Export the struct `ExtensionFilter`. Filter `DirEntry` items where the
    file extension is not in a list of allowed ones.
-   **rdr**: Export the struct `ReadDirRecursive`. Iterator similar to the standard `fs::ReadDir`
    but recursive.
-   **result_filter**: Export the struct `ResultFilter`. It maps an iterator over items of type
    `Result<T>` into one over items of type `T` by discarding `Err` variants.

## Generate and open the documentation

```bash
cargo doc --lib --open
```
