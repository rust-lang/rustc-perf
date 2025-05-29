pub fn code() {
    println!("Hello from dep_443");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_443");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_443: {t}");
}

pub fn foo() {
    dep_377::code();
    dep_377::code_inlined();
    dep_377::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
}
