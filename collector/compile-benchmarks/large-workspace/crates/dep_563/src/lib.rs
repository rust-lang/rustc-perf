pub fn code() {
    println!("Hello from dep_563");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_563");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_563: {t}");
}

pub fn foo() {
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
}
