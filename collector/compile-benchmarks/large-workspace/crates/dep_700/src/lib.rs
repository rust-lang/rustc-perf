pub fn code() {
    println!("Hello from dep_700");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_700");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_700: {t}");
}

pub fn foo() {
    dep_214::code();
    dep_214::code_inlined();
    dep_214::code_generic(1u32);
}
