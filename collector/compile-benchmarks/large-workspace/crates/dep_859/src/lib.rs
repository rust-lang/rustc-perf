pub fn code() {
    println!("Hello from dep_859");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_859");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_859: {t}");
}

pub fn foo() {
    dep_276::code();
    dep_276::code_inlined();
    dep_276::code_generic(1u32);
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
    dep_250::code();
    dep_250::code_inlined();
    dep_250::code_generic(1u32);
}
