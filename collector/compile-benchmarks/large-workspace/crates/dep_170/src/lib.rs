pub fn code() {
    println!("Hello from dep_170");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_170");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_170: {t}");
}

pub fn foo() {
    dep_110::code();
    dep_110::code_inlined();
    dep_110::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
}
