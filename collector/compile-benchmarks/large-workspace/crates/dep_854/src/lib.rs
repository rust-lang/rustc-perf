pub fn code() {
    println!("Hello from dep_854");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_854");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_854: {t}");
}

pub fn foo() {
    dep_318::code();
    dep_318::code_inlined();
    dep_318::code_generic(1u32);
    dep_405::code();
    dep_405::code_inlined();
    dep_405::code_generic(1u32);
    dep_166::code();
    dep_166::code_inlined();
    dep_166::code_generic(1u32);
    dep_219::code();
    dep_219::code_inlined();
    dep_219::code_generic(1u32);
    dep_369::code();
    dep_369::code_inlined();
    dep_369::code_generic(1u32);
}
