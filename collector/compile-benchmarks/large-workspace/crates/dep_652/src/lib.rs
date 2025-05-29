pub fn code() {
    println!("Hello from dep_652");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_652");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_652: {t}");
}

pub fn foo() {
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_262::code();
    dep_262::code_inlined();
    dep_262::code_generic(1u32);
}
