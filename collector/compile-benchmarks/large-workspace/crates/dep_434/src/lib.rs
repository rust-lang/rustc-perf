pub fn code() {
    println!("Hello from dep_434");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_434");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_434: {t}");
}

pub fn foo() {
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
}
