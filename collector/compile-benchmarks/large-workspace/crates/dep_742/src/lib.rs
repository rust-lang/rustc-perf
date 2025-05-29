pub fn code() {
    println!("Hello from dep_742");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_742");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_742: {t}");
}

pub fn foo() {
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_340::code();
    dep_340::code_inlined();
    dep_340::code_generic(1u32);
}
