pub fn code() {
    println!("Hello from dep_156");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_156");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_156: {t}");
}

pub fn foo() {
    dep_27::code();
    dep_27::code_inlined();
    dep_27::code_generic(1u32);
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
}
