pub fn code() {
    println!("Hello from dep_94");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_94");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_94: {t}");
}

pub fn foo() {
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
}
