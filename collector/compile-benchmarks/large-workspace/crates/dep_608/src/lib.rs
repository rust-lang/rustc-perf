pub fn code() {
    println!("Hello from dep_608");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_608");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_608: {t}");
}

pub fn foo() {
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
    dep_369::code();
    dep_369::code_inlined();
    dep_369::code_generic(1u32);
    dep_188::code();
    dep_188::code_inlined();
    dep_188::code_generic(1u32);
}
