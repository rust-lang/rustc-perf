pub fn code() {
    println!("Hello from dep_579");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_579");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_579: {t}");
}

pub fn foo() {
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
}
