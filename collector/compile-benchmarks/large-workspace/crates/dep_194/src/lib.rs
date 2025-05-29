pub fn code() {
    println!("Hello from dep_194");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_194");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_194: {t}");
}

pub fn foo() {
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
}
