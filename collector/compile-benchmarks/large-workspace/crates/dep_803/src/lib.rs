pub fn code() {
    println!("Hello from dep_803");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_803");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_803: {t}");
}

pub fn foo() {
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
    dep_301::code();
    dep_301::code_inlined();
    dep_301::code_generic(1u32);
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
    dep_282::code();
    dep_282::code_inlined();
    dep_282::code_generic(1u32);
}
