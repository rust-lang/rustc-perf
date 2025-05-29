pub fn code() {
    println!("Hello from dep_165");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_165");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_165: {t}");
}

pub fn foo() {
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_97::code();
    dep_97::code_inlined();
    dep_97::code_generic(1u32);
    dep_64::code();
    dep_64::code_inlined();
    dep_64::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
}
