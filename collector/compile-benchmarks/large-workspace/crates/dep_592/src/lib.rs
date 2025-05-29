pub fn code() {
    println!("Hello from dep_592");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_592");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_592: {t}");
}

pub fn foo() {
    dep_255::code();
    dep_255::code_inlined();
    dep_255::code_generic(1u32);
    dep_222::code();
    dep_222::code_inlined();
    dep_222::code_generic(1u32);
    dep_378::code();
    dep_378::code_inlined();
    dep_378::code_generic(1u32);
    dep_188::code();
    dep_188::code_inlined();
    dep_188::code_generic(1u32);
}
