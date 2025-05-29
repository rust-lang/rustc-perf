pub fn code() {
    println!("Hello from dep_197");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_197");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_197: {t}");
}

pub fn foo() {
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
    dep_91::code();
    dep_91::code_inlined();
    dep_91::code_generic(1u32);
    dep_69::code();
    dep_69::code_inlined();
    dep_69::code_generic(1u32);
    dep_148::code();
    dep_148::code_inlined();
    dep_148::code_generic(1u32);
}
