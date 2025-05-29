pub fn code() {
    println!("Hello from dep_562");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_562");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_562: {t}");
}

pub fn foo() {
    dep_340::code();
    dep_340::code_inlined();
    dep_340::code_generic(1u32);
    dep_200::code();
    dep_200::code_inlined();
    dep_200::code_generic(1u32);
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
}
