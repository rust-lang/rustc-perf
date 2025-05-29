pub fn code() {
    println!("Hello from dep_737");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_737");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_737: {t}");
}

pub fn foo() {
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
    dep_181::code();
    dep_181::code_inlined();
    dep_181::code_generic(1u32);
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
}
