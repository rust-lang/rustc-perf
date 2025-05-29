pub fn code() {
    println!("Hello from dep_202");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_202");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_202: {t}");
}

pub fn foo() {
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
}
