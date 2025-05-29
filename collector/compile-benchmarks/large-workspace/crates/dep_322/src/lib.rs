pub fn code() {
    println!("Hello from dep_322");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_322");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_322: {t}");
}

pub fn foo() {
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
}
