pub fn code() {
    println!("Hello from dep_148");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_148");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_148: {t}");
}

pub fn foo() {
    dep_33::code();
    dep_33::code_inlined();
    dep_33::code_generic(1u32);
}
