pub fn code() {
    println!("Hello from dep_187");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_187");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_187: {t}");
}

pub fn foo() {
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_148::code();
    dep_148::code_inlined();
    dep_148::code_generic(1u32);
}
