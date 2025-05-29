pub fn code() {
    println!("Hello from dep_403");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_403");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_403: {t}");
}

pub fn foo() {
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
}
