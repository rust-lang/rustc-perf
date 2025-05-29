pub fn code() {
    println!("Hello from dep_472");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_472");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_472: {t}");
}

pub fn foo() {
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_316::code();
    dep_316::code_inlined();
    dep_316::code_generic(1u32);
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
}
