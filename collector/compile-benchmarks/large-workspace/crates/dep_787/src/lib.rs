pub fn code() {
    println!("Hello from dep_787");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_787");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_787: {t}");
}

pub fn foo() {
    dep_394::code();
    dep_394::code_inlined();
    dep_394::code_generic(1u32);
    dep_406::code();
    dep_406::code_inlined();
    dep_406::code_generic(1u32);
}
