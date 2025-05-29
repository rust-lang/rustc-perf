pub fn code() {
    println!("Hello from dep_256");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_256");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_256: {t}");
}

pub fn foo() {
    dep_131::code();
    dep_131::code_inlined();
    dep_131::code_generic(1u32);
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
}
