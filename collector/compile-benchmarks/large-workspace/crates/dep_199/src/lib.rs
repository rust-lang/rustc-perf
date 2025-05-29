pub fn code() {
    println!("Hello from dep_199");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_199");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_199: {t}");
}

pub fn foo() {
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
}
