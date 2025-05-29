pub fn code() {
    println!("Hello from dep_375");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_375");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_375: {t}");
}

pub fn foo() {
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_73::code();
    dep_73::code_inlined();
    dep_73::code_generic(1u32);
}
