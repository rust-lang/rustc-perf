pub fn code() {
    println!("Hello from dep_529");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_529");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_529: {t}");
}

pub fn foo() {
    dep_334::code();
    dep_334::code_inlined();
    dep_334::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
}
