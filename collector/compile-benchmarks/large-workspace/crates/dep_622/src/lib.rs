pub fn code() {
    println!("Hello from dep_622");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_622");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_622: {t}");
}

pub fn foo() {
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
    dep_175::code();
    dep_175::code_inlined();
    dep_175::code_generic(1u32);
    dep_356::code();
    dep_356::code_inlined();
    dep_356::code_generic(1u32);
}
