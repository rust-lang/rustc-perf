pub fn code() {
    println!("Hello from dep_612");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_612");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_612: {t}");
}

pub fn foo() {
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_356::code();
    dep_356::code_inlined();
    dep_356::code_generic(1u32);
    dep_315::code();
    dep_315::code_inlined();
    dep_315::code_generic(1u32);
}
