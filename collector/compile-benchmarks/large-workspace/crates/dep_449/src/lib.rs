pub fn code() {
    println!("Hello from dep_449");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_449");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_449: {t}");
}

pub fn foo() {
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
}
