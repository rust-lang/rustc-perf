pub fn code() {
    println!("Hello from dep_806");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_806");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_806: {t}");
}

pub fn foo() {
    dep_308::code();
    dep_308::code_inlined();
    dep_308::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_347::code();
    dep_347::code_inlined();
    dep_347::code_generic(1u32);
}
