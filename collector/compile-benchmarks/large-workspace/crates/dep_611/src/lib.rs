pub fn code() {
    println!("Hello from dep_611");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_611");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_611: {t}");
}

pub fn foo() {
    dep_162::code();
    dep_162::code_inlined();
    dep_162::code_generic(1u32);
    dep_386::code();
    dep_386::code_inlined();
    dep_386::code_generic(1u32);
}
