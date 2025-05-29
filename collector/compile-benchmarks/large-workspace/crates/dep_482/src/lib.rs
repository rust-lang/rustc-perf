pub fn code() {
    println!("Hello from dep_482");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_482");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_482: {t}");
}

pub fn foo() {
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_162::code();
    dep_162::code_inlined();
    dep_162::code_generic(1u32);
}
