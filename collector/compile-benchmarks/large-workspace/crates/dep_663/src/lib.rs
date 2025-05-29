pub fn code() {
    println!("Hello from dep_663");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_663");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_663: {t}");
}

pub fn foo() {
    dep_213::code();
    dep_213::code_inlined();
    dep_213::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_389::code();
    dep_389::code_inlined();
    dep_389::code_generic(1u32);
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
}
