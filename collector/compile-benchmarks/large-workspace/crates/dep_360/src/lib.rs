pub fn code() {
    println!("Hello from dep_360");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_360");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_360: {t}");
}

pub fn foo() {
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_91::code();
    dep_91::code_inlined();
    dep_91::code_generic(1u32);
    dep_126::code();
    dep_126::code_inlined();
    dep_126::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
}
