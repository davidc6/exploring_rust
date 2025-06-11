# Generics

Generics enable to build types from other types without knowing all possible type combinations. Generics are compile-type abstraction so compile time is increased. Generics are similar to C++ templates.

Since Rust's type system is Turing-complete (i.e. anything computable can be computed) meaning that anything can be computed at compile type (using the compiler as a CPU).

```rs
// 1. Generics example

struct Container<T> {
    value: T,
}
```


```rs
// 2. Generics example

enum FileSystemType {
    Fat,
    Ntfs,
    Ext4,
    Hfs
}

struct FileSystem {
    fs_type: FileSystemType
}

impl FileSystem {
    // This is the same as
    // pub fn check_support<T: Support>(&self, os: T) -> bool { ... }
    pub fn check_support(&self, os: impl Support) -> bool {
        os.is_fs_supported(self.fs_type)
    }
}
```

### const generics

- https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
- https://practice.course.rs/generics-traits/const-generics.html
