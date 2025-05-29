pub fn code() {
    println!("Hello from dep_502");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_502");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_502: {t}");
}

pub fn foo() {
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
    dep_178::code();
    dep_178::code_inlined();
    dep_178::code_generic(1u32);
}
