pub fn code() {
    println!("Hello from dep_66");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_66");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_66: {t}");
}

pub fn foo() {
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
    dep_49::code();
    dep_49::code_inlined();
    dep_49::code_generic(1u32);
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_15::code();
    dep_15::code_inlined();
    dep_15::code_generic(1u32);
    dep_33::code();
    dep_33::code_inlined();
    dep_33::code_generic(1u32);
}
