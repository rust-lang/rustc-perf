pub fn code() {
    println!("Hello from dep_550");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_550");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_550: {t}");
}

pub fn foo() {
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
    dep_207::code();
    dep_207::code_inlined();
    dep_207::code_generic(1u32);
    dep_389::code();
    dep_389::code_inlined();
    dep_389::code_generic(1u32);
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
}
