pub fn code() {
    println!("Hello from dep_327");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_327");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_327: {t}");
}

pub fn foo() {
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
}
