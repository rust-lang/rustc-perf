pub fn code() {
    println!("Hello from dep_896");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_896");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_896: {t}");
}

pub fn foo() {
    dep_301::code();
    dep_301::code_inlined();
    dep_301::code_generic(1u32);
    dep_261::code();
    dep_261::code_inlined();
    dep_261::code_generic(1u32);
    dep_302::code();
    dep_302::code_inlined();
    dep_302::code_generic(1u32);
    dep_378::code();
    dep_378::code_inlined();
    dep_378::code_generic(1u32);
}
