pub fn code() {
    println!("Hello from dep_465");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_465");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_465: {t}");
}

pub fn foo() {
    dep_389::code();
    dep_389::code_inlined();
    dep_389::code_generic(1u32);
    dep_278::code();
    dep_278::code_inlined();
    dep_278::code_generic(1u32);
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
}
