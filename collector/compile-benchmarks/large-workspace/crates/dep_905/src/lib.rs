pub fn code() {
    println!("Hello from dep_905");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_905");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_905: {t}");
}

pub fn foo() {
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
    dep_286::code();
    dep_286::code_inlined();
    dep_286::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
    dep_187::code();
    dep_187::code_inlined();
    dep_187::code_generic(1u32);
}
