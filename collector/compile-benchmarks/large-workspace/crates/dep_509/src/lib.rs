pub fn code() {
    println!("Hello from dep_509");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_509");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_509: {t}");
}

pub fn foo() {
    dep_373::code();
    dep_373::code_inlined();
    dep_373::code_generic(1u32);
    dep_177::code();
    dep_177::code_inlined();
    dep_177::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
    dep_385::code();
    dep_385::code_inlined();
    dep_385::code_generic(1u32);
}
