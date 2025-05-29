pub fn code() {
    println!("Hello from dep_450");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_450");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_450: {t}");
}

pub fn foo() {
    dep_219::code();
    dep_219::code_inlined();
    dep_219::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
}
