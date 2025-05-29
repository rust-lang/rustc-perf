pub fn code() {
    println!("Hello from dep_661");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_661");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_661: {t}");
}

pub fn foo() {
    dep_239::code();
    dep_239::code_inlined();
    dep_239::code_generic(1u32);
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
}
