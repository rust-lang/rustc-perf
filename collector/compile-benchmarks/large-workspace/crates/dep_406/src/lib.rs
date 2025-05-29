pub fn code() {
    println!("Hello from dep_406");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_406");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_406: {t}");
}

pub fn foo() {
    dep_112::code();
    dep_112::code_inlined();
    dep_112::code_generic(1u32);
}
