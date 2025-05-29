pub fn code() {
    println!("Hello from dep_752");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_752");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_752: {t}");
}

pub fn foo() {
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
    dep_219::code();
    dep_219::code_inlined();
    dep_219::code_generic(1u32);
}
