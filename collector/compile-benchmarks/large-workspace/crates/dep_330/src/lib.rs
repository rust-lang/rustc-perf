pub fn code() {
    println!("Hello from dep_330");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_330");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_330: {t}");
}

pub fn foo() {
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_91::code();
    dep_91::code_inlined();
    dep_91::code_generic(1u32);
    dep_146::code();
    dep_146::code_inlined();
    dep_146::code_generic(1u32);
}
