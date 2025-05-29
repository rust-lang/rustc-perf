pub fn code() {
    println!("Hello from dep_227");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_227");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_227: {t}");
}

pub fn foo() {
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_100::code();
    dep_100::code_inlined();
    dep_100::code_generic(1u32);
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
}
