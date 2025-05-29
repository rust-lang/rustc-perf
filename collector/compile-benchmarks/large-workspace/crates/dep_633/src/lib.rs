pub fn code() {
    println!("Hello from dep_633");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_633");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_633: {t}");
}

pub fn foo() {
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
    dep_187::code();
    dep_187::code_inlined();
    dep_187::code_generic(1u32);
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
}
