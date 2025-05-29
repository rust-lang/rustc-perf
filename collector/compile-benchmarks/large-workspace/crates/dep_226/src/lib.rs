pub fn code() {
    println!("Hello from dep_226");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_226");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_226: {t}");
}

pub fn foo() {
    dep_134::code();
    dep_134::code_inlined();
    dep_134::code_generic(1u32);
    dep_139::code();
    dep_139::code_inlined();
    dep_139::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
}
