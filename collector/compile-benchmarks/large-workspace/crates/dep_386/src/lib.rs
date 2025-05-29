pub fn code() {
    println!("Hello from dep_386");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_386");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_386: {t}");
}

pub fn foo() {
    dep_73::code();
    dep_73::code_inlined();
    dep_73::code_generic(1u32);
    dep_70::code();
    dep_70::code_inlined();
    dep_70::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
}
