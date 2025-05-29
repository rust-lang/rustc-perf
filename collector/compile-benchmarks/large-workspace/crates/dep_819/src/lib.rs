pub fn code() {
    println!("Hello from dep_819");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_819");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_819: {t}");
}

pub fn foo() {
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_369::code();
    dep_369::code_inlined();
    dep_369::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
}
