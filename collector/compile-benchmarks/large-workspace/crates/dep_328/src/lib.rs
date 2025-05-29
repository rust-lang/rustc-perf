pub fn code() {
    println!("Hello from dep_328");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_328");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_328: {t}");
}

pub fn foo() {
    dep_73::code();
    dep_73::code_inlined();
    dep_73::code_generic(1u32);
}
