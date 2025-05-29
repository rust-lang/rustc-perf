pub fn code() {
    println!("Hello from dep_747");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_747");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_747: {t}");
}

pub fn foo() {
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_177::code();
    dep_177::code_inlined();
    dep_177::code_generic(1u32);
    dep_217::code();
    dep_217::code_inlined();
    dep_217::code_generic(1u32);
}
