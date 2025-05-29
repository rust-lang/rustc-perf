pub fn code() {
    println!("Hello from dep_246");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_246");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_246: {t}");
}

pub fn foo() {
    dep_126::code();
    dep_126::code_inlined();
    dep_126::code_generic(1u32);
    dep_118::code();
    dep_118::code_inlined();
    dep_118::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
}
