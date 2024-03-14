# Enzyme-Rust Book

This book is build using [mdBook](https://rust-lang.github.io/mdBook/index.html)

It requires a working rustup installation.
Install mdBook with

```console
$ cargo install mdbook
```

Build and run this book afterwards locally with
```console
$ mdbook serve --open
```

## Running tests

This book contains many code fragments that are important to test.

### Rust-Enzyme toolchain
We assume that you have a rustup toolchain called `enzyme`. This would have been created in your rust build directory by running something like:

```console
$ rustup toolchain link enzyme $(pwd)/build/host/stage1
```

The above only needs to be run once. The `enzyme` toolchain will also need `rustdoc` support. Again, from the Rust build directory, you'll need to have run something like

```console
$ x build --stage 1 library tools/rustdoc
```

This will need to be re-run each time you `git pull`.

### Configuring an override

It's unlikely that you want `enzyme` as your default toolchain for all your rust activities. Instead, you probably use `cargo +enzyme build` when working with Rust-Enyzme. You can specify a [directory override](https://rust-lang.github.io/rustup/overrides.html#directory-overrides) by running the following (once) in the Enzyme-rustbook directory:

```console
$ rustup override set enzyme
```

### mdBook testing

Finally, you can run tests for all chapters using

```console
$ mdbook test
```

or a specific chapter using that chapter's name, such as

```console
$ mdbook test -c Usage
```

#### mdBook and `lto=fat` (temporary workaround)

Rust's Enzyme support currently requires `lto=fat`, which is not part of the default profile for `rustdoc` (which is invoked by `mdbook test`). My temporary hack has been to patch `mdbook` by applying the following.

``` diff
diff --git i/src/book/mod.rs w/src/book/mod.rs
index c0ab8a5..409482f 100644
--- i/src/book/mod.rs
+++ w/src/book/mod.rs
@@ -319,7 +319,11 @@ impl MDBook {
                 tmpf.write_all(ch.content.as_bytes())?;
 
                 let mut cmd = Command::new("rustdoc");
-                cmd.arg(&path).arg("--test").args(&library_args);
+                cmd.arg(&path)
+                    .arg("--test")
+                    .args(&library_args)
+                    .arg("-C")
+                    .arg("lto=fat");
 
                 if let Some(edition) = self.config.rust.edition {
                     match edition {
```

I'll find a better solution and make a pull request.
