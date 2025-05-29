pub fn code() {
    println!("Hello from dep_358");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_358");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_358: {t}");
}

pub fn foo() {
    dep_155::code();
    dep_155::code_inlined();
    dep_155::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
}
