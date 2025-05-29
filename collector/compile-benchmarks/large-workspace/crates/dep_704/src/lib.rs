pub fn code() {
    println!("Hello from dep_704");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_704");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_704: {t}");
}

pub fn foo() {
    dep_198::code();
    dep_198::code_inlined();
    dep_198::code_generic(1u32);
}
