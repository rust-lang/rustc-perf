pub fn code() {
    println!("Hello from dep_839");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_839");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_839: {t}");
}

pub fn foo() {
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_314::code();
    dep_314::code_inlined();
    dep_314::code_generic(1u32);
}
