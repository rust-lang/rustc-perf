pub fn code() {
    println!("Hello from dep_219");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_219");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_219: {t}");
}

pub fn foo() {
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
    dep_139::code();
    dep_139::code_inlined();
    dep_139::code_generic(1u32);
    dep_151::code();
    dep_151::code_inlined();
    dep_151::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
}
