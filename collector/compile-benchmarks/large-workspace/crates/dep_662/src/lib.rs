pub fn code() {
    println!("Hello from dep_662");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_662");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_662: {t}");
}

pub fn foo() {
    dep_181::code();
    dep_181::code_inlined();
    dep_181::code_generic(1u32);
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
}
