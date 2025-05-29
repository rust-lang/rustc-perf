pub fn code() {
    println!("Hello from dep_630");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_630");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_630: {t}");
}

pub fn foo() {
    dep_278::code();
    dep_278::code_inlined();
    dep_278::code_generic(1u32);
}
