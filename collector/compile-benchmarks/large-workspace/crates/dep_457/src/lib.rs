pub fn code() {
    println!("Hello from dep_457");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_457");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_457: {t}");
}

pub fn foo() {
    dep_235::code();
    dep_235::code_inlined();
    dep_235::code_generic(1u32);
    dep_204::code();
    dep_204::code_inlined();
    dep_204::code_generic(1u32);
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
    dep_170::code();
    dep_170::code_inlined();
    dep_170::code_generic(1u32);
}
