pub fn code() {
    println!("Hello from dep_494");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_494");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_494: {t}");
}

pub fn foo() {
    dep_223::code();
    dep_223::code_inlined();
    dep_223::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
}
