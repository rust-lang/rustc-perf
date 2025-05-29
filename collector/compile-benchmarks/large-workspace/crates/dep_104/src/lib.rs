pub fn code() {
    println!("Hello from dep_104");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_104");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_104: {t}");
}

pub fn foo() {
    dep_22::code();
    dep_22::code_inlined();
    dep_22::code_generic(1u32);
    dep_47::code();
    dep_47::code_inlined();
    dep_47::code_generic(1u32);
}
