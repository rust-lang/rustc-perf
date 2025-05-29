pub fn code() {
    println!("Hello from dep_730");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_730");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_730: {t}");
}

pub fn foo() {
    dep_178::code();
    dep_178::code_inlined();
    dep_178::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
}
