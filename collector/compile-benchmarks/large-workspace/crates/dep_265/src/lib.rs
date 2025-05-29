pub fn code() {
    println!("Hello from dep_265");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_265");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_265: {t}");
}

pub fn foo() {
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
}
