pub fn code() {
    println!("Hello from dep_897");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_897");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_897: {t}");
}

pub fn foo() {
    dep_173::code();
    dep_173::code_inlined();
    dep_173::code_generic(1u32);
    dep_183::code();
    dep_183::code_inlined();
    dep_183::code_generic(1u32);
}
