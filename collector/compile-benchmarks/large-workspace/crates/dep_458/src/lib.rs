pub fn code() {
    println!("Hello from dep_458");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_458");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_458: {t}");
}

pub fn foo() {
    dep_229::code();
    dep_229::code_inlined();
    dep_229::code_generic(1u32);
    dep_217::code();
    dep_217::code_inlined();
    dep_217::code_generic(1u32);
}
