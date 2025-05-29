pub fn code() {
    println!("Hello from dep_283");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_283");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_283: {t}");
}

pub fn foo() {
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
}
