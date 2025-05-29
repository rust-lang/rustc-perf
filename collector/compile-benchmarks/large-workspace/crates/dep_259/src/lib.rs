pub fn code() {
    println!("Hello from dep_259");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_259");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_259: {t}");
}

pub fn foo() {
    dep_158::code();
    dep_158::code_inlined();
    dep_158::code_generic(1u32);
}
