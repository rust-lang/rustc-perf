pub fn code() {
    println!("Hello from dep_585");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_585");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_585: {t}");
}

pub fn foo() {
    dep_386::code();
    dep_386::code_inlined();
    dep_386::code_generic(1u32);
    dep_200::code();
    dep_200::code_inlined();
    dep_200::code_generic(1u32);
    dep_377::code();
    dep_377::code_inlined();
    dep_377::code_generic(1u32);
    dep_366::code();
    dep_366::code_inlined();
    dep_366::code_generic(1u32);
}
