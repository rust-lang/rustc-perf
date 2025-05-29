pub fn code() {
    println!("Hello from dep_169");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_169");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_169: {t}");
}

pub fn foo() {
    dep_61::code();
    dep_61::code_inlined();
    dep_61::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
}
