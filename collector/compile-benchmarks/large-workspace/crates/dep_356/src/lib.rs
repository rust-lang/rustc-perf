pub fn code() {
    println!("Hello from dep_356");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_356");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_356: {t}");
}

pub fn foo() {
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
}
