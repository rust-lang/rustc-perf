pub fn code() {
    println!("Hello from dep_234");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_234");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_234: {t}");
}

pub fn foo() {
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
    dep_148::code();
    dep_148::code_inlined();
    dep_148::code_generic(1u32);
}
