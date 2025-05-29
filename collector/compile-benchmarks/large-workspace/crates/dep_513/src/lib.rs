pub fn code() {
    println!("Hello from dep_513");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_513");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_513: {t}");
}

pub fn foo() {
    dep_360::code();
    dep_360::code_inlined();
    dep_360::code_generic(1u32);
    dep_378::code();
    dep_378::code_inlined();
    dep_378::code_generic(1u32);
    dep_406::code();
    dep_406::code_inlined();
    dep_406::code_generic(1u32);
}
