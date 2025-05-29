pub fn code() {
    println!("Hello from dep_890");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_890");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_890: {t}");
}

pub fn foo() {
    dep_238::code();
    dep_238::code_inlined();
    dep_238::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
    dep_357::code();
    dep_357::code_inlined();
    dep_357::code_generic(1u32);
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
}
