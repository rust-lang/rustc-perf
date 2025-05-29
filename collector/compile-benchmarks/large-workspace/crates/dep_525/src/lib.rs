pub fn code() {
    println!("Hello from dep_525");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_525");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_525: {t}");
}

pub fn foo() {
    dep_166::code();
    dep_166::code_inlined();
    dep_166::code_generic(1u32);
    dep_237::code();
    dep_237::code_inlined();
    dep_237::code_generic(1u32);
    dep_223::code();
    dep_223::code_inlined();
    dep_223::code_generic(1u32);
    dep_331::code();
    dep_331::code_inlined();
    dep_331::code_generic(1u32);
    dep_214::code();
    dep_214::code_inlined();
    dep_214::code_generic(1u32);
}
