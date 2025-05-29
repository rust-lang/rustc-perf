pub fn code() {
    println!("Hello from dep_264");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_264");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_264: {t}");
}

pub fn foo() {
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
}
