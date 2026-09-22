A deep `Deref` chain where every target type defines a method with the same
name, so many sidebar links want the same anchor and rustdoc has to make each
one unique.

See rust-lang/rust#158174 and rust-lang/rust#162976

Generated with (N = 800):
```python
    N = 800
    with open("src/lib.rs", "w") as f:
        for i in range(N + 1):
            f.write(f"pub struct S{i};\n")
        for i in range(N + 1):
            f.write(f"impl S{i} {{ pub fn foo(&self) {{}} }}\n")
        for i in range(N):
            j = i + 1
            f.write(
                f"impl std::ops::Deref for S{i} {{ "
                f"type Target = S{j}; "
                f"fn deref(&self) -> &S{j} {{ static V: S{j} = S{j}; &V }} "
                "}\n"
            )
```
