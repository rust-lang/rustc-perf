pub fn code() {
    println!("Hello from dep_686");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_686");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_686: {t}");
}

pub fn foo() {
    dep_312::code();
    dep_312::code_inlined();
    dep_312::code_generic(1u32);
    dep_378::code();
    dep_378::code_inlined();
    dep_378::code_generic(1u32);
    dep_282::code();
    dep_282::code_inlined();
    dep_282::code_generic(1u32);
}
