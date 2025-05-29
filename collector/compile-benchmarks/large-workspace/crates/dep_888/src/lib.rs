pub fn code() {
    println!("Hello from dep_888");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_888");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_888: {t}");
}

pub fn foo() {
    dep_287::code();
    dep_287::code_inlined();
    dep_287::code_generic(1u32);
    dep_202::code();
    dep_202::code_inlined();
    dep_202::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
    dep_314::code();
    dep_314::code_inlined();
    dep_314::code_generic(1u32);
}
