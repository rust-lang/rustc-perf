pub fn code() {
    println!("Hello from dep_637");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_637");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_637: {t}");
}

pub fn foo() {
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
}
