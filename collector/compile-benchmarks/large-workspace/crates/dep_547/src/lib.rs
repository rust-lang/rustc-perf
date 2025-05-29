pub fn code() {
    println!("Hello from dep_547");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_547");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_547: {t}");
}

pub fn foo() {
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
}
