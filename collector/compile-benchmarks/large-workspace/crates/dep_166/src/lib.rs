pub fn code() {
    println!("Hello from dep_166");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_166");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_166: {t}");
}

pub fn foo() {
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
}
