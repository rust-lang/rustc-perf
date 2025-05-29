pub fn code() {
    println!("Hello from dep_405");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_405");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_405: {t}");
}

pub fn foo() {
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
}
