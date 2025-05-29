pub fn code() {
    println!("Hello from dep_476");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_476");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_476: {t}");
}

pub fn foo() {
    dep_367::code();
    dep_367::code_inlined();
    dep_367::code_generic(1u32);
}
