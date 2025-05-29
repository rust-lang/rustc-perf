pub fn code() {
    println!("Hello from dep_826");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_826");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_826: {t}");
}

pub fn foo() {
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
    dep_369::code();
    dep_369::code_inlined();
    dep_369::code_generic(1u32);
}
