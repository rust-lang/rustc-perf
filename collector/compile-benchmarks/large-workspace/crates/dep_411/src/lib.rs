pub fn code() {
    println!("Hello from dep_411");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_411");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_411: {t}");
}

pub fn foo() {
    dep_327::code();
    dep_327::code_inlined();
    dep_327::code_generic(1u32);
    dep_270::code();
    dep_270::code_inlined();
    dep_270::code_generic(1u32);
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
}
