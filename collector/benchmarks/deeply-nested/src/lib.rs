// A test case from rust-lang/rust#38528.
// The original blowup had both exponential compute and memory complexity.
// The nesting should be kept at 16 level to avoid crashing the test machine.

pub fn foo() -> Box<Iterator<Item = ()>> {
    use std::iter::empty;

    Box::new(empty()
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty()) // 10th .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty()) // 16th .chain(empty())
    )
}
