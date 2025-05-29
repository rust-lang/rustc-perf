pub fn code() {
    println!("Hello from dep_531");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_531");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_531: {t}");
}

pub fn foo() {
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_279::code();
    dep_279::code_inlined();
    dep_279::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
}
