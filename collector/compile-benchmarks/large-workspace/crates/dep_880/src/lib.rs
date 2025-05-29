pub fn code() {
    println!("Hello from dep_880");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_880");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_880: {t}");
}

pub fn foo() {
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
}
