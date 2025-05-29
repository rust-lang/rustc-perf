pub fn code() {
    println!("Hello from dep_384");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_384");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_384: {t}");
}

pub fn foo() {
    dep_94::code();
    dep_94::code_inlined();
    dep_94::code_generic(1u32);
    dep_121::code();
    dep_121::code_inlined();
    dep_121::code_generic(1u32);
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
}
