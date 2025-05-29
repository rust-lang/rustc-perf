pub fn code() {
    println!("Hello from dep_366");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_366");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_366: {t}");
}

pub fn foo() {
    dep_103::code();
    dep_103::code_inlined();
    dep_103::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
}
