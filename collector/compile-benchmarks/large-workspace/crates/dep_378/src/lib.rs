pub fn code() {
    println!("Hello from dep_378");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_378");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_378: {t}");
}

pub fn foo() {
    dep_60::code();
    dep_60::code_inlined();
    dep_60::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
    dep_105::code();
    dep_105::code_inlined();
    dep_105::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
}
