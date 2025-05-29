pub fn code() {
    println!("Hello from dep_391");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_391");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_391: {t}");
}

pub fn foo() {
    dep_146::code();
    dep_146::code_inlined();
    dep_146::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
}
