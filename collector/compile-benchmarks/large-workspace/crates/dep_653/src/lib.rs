pub fn code() {
    println!("Hello from dep_653");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_653");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_653: {t}");
}

pub fn foo() {
    dep_340::code();
    dep_340::code_inlined();
    dep_340::code_generic(1u32);
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
    dep_217::code();
    dep_217::code_inlined();
    dep_217::code_generic(1u32);
}
