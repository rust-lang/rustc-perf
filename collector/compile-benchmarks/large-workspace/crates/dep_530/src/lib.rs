pub fn code() {
    println!("Hello from dep_530");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_530");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_530: {t}");
}

pub fn foo() {
    dep_304::code();
    dep_304::code_inlined();
    dep_304::code_generic(1u32);
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
    dep_378::code();
    dep_378::code_inlined();
    dep_378::code_generic(1u32);
    dep_204::code();
    dep_204::code_inlined();
    dep_204::code_generic(1u32);
}
