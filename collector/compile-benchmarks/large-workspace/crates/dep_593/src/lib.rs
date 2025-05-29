pub fn code() {
    println!("Hello from dep_593");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_593");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_593: {t}");
}

pub fn foo() {
    dep_363::code();
    dep_363::code_inlined();
    dep_363::code_generic(1u32);
    dep_372::code();
    dep_372::code_inlined();
    dep_372::code_generic(1u32);
    dep_381::code();
    dep_381::code_inlined();
    dep_381::code_generic(1u32);
    dep_312::code();
    dep_312::code_inlined();
    dep_312::code_generic(1u32);
}
