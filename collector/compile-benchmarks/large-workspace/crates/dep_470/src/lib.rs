pub fn code() {
    println!("Hello from dep_470");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_470");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_470: {t}");
}

pub fn foo() {
    dep_179::code();
    dep_179::code_inlined();
    dep_179::code_generic(1u32);
    dep_285::code();
    dep_285::code_inlined();
    dep_285::code_generic(1u32);
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
}
