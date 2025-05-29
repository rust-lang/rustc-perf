pub fn code() {
    println!("Hello from dep_210");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_210");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_210: {t}");
}

pub fn foo() {
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
}
