pub fn code() {
    println!("Hello from dep_268");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_268");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_268: {t}");
}

pub fn foo() {
    dep_73::code();
    dep_73::code_inlined();
    dep_73::code_generic(1u32);
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
}
