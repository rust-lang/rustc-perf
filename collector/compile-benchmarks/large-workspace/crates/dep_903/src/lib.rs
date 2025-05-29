pub fn code() {
    println!("Hello from dep_903");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_903");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_903: {t}");
}

pub fn foo() {
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
    dep_188::code();
    dep_188::code_inlined();
    dep_188::code_generic(1u32);
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
    dep_207::code();
    dep_207::code_inlined();
    dep_207::code_generic(1u32);
}
