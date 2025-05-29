pub fn code() {
    println!("Hello from dep_691");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_691");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_691: {t}");
}

pub fn foo() {
    dep_274::code();
    dep_274::code_inlined();
    dep_274::code_generic(1u32);
    dep_233::code();
    dep_233::code_inlined();
    dep_233::code_generic(1u32);
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
}
