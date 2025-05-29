pub fn code() {
    println!("Hello from dep_446");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_446");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_446: {t}");
}

pub fn foo() {
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
    dep_345::code();
    dep_345::code_inlined();
    dep_345::code_generic(1u32);
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_398::code();
    dep_398::code_inlined();
    dep_398::code_generic(1u32);
}
