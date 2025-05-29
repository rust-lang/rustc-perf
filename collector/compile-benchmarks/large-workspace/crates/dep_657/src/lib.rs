pub fn code() {
    println!("Hello from dep_657");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_657");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_657: {t}");
}

pub fn foo() {
    dep_389::code();
    dep_389::code_inlined();
    dep_389::code_generic(1u32);
}
