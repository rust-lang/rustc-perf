pub fn code() {
    println!("Hello from dep_744");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_744");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_744: {t}");
}

pub fn foo() {
    dep_345::code();
    dep_345::code_inlined();
    dep_345::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
}
