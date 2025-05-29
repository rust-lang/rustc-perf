pub fn code() {
    println!("Hello from dep_270");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_270");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_270: {t}");
}

pub fn foo() {
    dep_139::code();
    dep_139::code_inlined();
    dep_139::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
    dep_146::code();
    dep_146::code_inlined();
    dep_146::code_generic(1u32);
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
}
