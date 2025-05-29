pub fn code() {
    println!("Hello from dep_351");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_351");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_351: {t}");
}

pub fn foo() {
    dep_138::code();
    dep_138::code_inlined();
    dep_138::code_generic(1u32);
    dep_77::code();
    dep_77::code_inlined();
    dep_77::code_generic(1u32);
    dep_125::code();
    dep_125::code_inlined();
    dep_125::code_generic(1u32);
    dep_148::code();
    dep_148::code_inlined();
    dep_148::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
}
