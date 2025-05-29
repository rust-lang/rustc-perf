pub fn code() {
    println!("Hello from dep_589");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_589");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_589: {t}");
}

pub fn foo() {
    dep_253::code();
    dep_253::code_inlined();
    dep_253::code_generic(1u32);
    dep_383::code();
    dep_383::code_inlined();
    dep_383::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
}
