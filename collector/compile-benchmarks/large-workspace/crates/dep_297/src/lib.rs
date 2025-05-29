pub fn code() {
    println!("Hello from dep_297");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_297");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_297: {t}");
}

pub fn foo() {
    dep_112::code();
    dep_112::code_inlined();
    dep_112::code_generic(1u32);
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
}
