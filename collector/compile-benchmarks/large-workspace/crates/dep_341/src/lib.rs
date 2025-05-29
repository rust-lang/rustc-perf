pub fn code() {
    println!("Hello from dep_341");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_341");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_341: {t}");
}

pub fn foo() {
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
    dep_150::code();
    dep_150::code_inlined();
    dep_150::code_generic(1u32);
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
}
