pub fn code() {
    println!("Hello from dep_682");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_682");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_682: {t}");
}

pub fn foo() {
    dep_175::code();
    dep_175::code_inlined();
    dep_175::code_generic(1u32);
}
