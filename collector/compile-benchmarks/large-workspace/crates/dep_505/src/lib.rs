pub fn code() {
    println!("Hello from dep_505");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_505");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_505: {t}");
}

pub fn foo() {
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
    dep_186::code();
    dep_186::code_inlined();
    dep_186::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
}
