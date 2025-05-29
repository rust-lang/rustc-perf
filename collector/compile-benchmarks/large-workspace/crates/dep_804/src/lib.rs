pub fn code() {
    println!("Hello from dep_804");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_804");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_804: {t}");
}

pub fn foo() {
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
    dep_214::code();
    dep_214::code_inlined();
    dep_214::code_generic(1u32);
    dep_401::code();
    dep_401::code_inlined();
    dep_401::code_generic(1u32);
}
