pub fn code() {
    println!("Hello from dep_654");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_654");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_654: {t}");
}

pub fn foo() {
    dep_401::code();
    dep_401::code_inlined();
    dep_401::code_generic(1u32);
}
