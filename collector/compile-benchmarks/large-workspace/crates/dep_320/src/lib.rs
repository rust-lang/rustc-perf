pub fn code() {
    println!("Hello from dep_320");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_320");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_320: {t}");
}

pub fn foo() {
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
}
