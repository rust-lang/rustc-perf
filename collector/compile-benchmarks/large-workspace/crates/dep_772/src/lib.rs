pub fn code() {
    println!("Hello from dep_772");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_772");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_772: {t}");
}

pub fn foo() {
    dep_252::code();
    dep_252::code_inlined();
    dep_252::code_generic(1u32);
    dep_325::code();
    dep_325::code_inlined();
    dep_325::code_generic(1u32);
}
