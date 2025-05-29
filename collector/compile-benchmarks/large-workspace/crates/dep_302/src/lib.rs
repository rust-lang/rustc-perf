pub fn code() {
    println!("Hello from dep_302");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_302");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_302: {t}");
}

pub fn foo() {
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
    dep_154::code();
    dep_154::code_inlined();
    dep_154::code_generic(1u32);
}
