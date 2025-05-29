pub fn code() {
    println!("Hello from dep_619");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_619");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_619: {t}");
}

pub fn foo() {
    dep_403::code();
    dep_403::code_inlined();
    dep_403::code_generic(1u32);
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_266::code();
    dep_266::code_inlined();
    dep_266::code_generic(1u32);
}
