pub fn code() {
    println!("Hello from dep_353");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_353");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_353: {t}");
}

pub fn foo() {
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_103::code();
    dep_103::code_inlined();
    dep_103::code_generic(1u32);
}
