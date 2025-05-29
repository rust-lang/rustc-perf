pub fn code() {
    println!("Hello from dep_279");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_279");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_279: {t}");
}

pub fn foo() {
    dep_121::code();
    dep_121::code_inlined();
    dep_121::code_generic(1u32);
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
}
