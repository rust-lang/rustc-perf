pub fn code() {
    println!("Hello from dep_73");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_73");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_73: {t}");
}

pub fn foo() {
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_25::code();
    dep_25::code_inlined();
    dep_25::code_generic(1u32);
    dep_29::code();
    dep_29::code_inlined();
    dep_29::code_generic(1u32);
    dep_31::code();
    dep_31::code_inlined();
    dep_31::code_generic(1u32);
}
