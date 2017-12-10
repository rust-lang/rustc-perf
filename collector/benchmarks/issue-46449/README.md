Performance test to track <https://github.com/rust-lang/rust/issues/46449>.

| Patch             | 1.22.1        | 1.23.0-beta.2 | Note                                                          |
|-------------------|---------------|---------------|---------------------------------------------------------------|
| (baseline)        | Fast (~1s)    | Fast (~1s)    | Make the depedencies available. Should finish very quickly.   |
| 0-io-error-6144   | Fast (~1s)    | Slow (~16s)   | This is the reduced test case of the original bug report      |
| 1-u32-3072        | Slow (~21s)   | Fast (~1s)    | Length reduced to 3072 to avoid taking too much time.         |
| 2-u8-3072         | Slow (~21s)   | Fast (~1s)    |                                                               |
| 3-empty-3072      | Slow (~20s)   | Slow (~27s)   |                                                               |
| 4-static-str-6144 | Slow (~22s)   | Medium (~8s)  | Length increased back to 6144 as this is faster.              |
