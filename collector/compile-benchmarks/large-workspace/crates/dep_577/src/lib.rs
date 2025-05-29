pub fn code() {
    println!("Hello from dep_577");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_577");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_577: {t}");
}

pub fn foo() {
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
}
