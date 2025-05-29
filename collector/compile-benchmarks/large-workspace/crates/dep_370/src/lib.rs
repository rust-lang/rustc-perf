pub fn code() {
    println!("Hello from dep_370");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_370");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_370: {t}");
}

pub fn foo() {
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_73::code();
    dep_73::code_inlined();
    dep_73::code_generic(1u32);
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
}
