pub fn code() {
    println!("Hello from dep_135");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_135");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_135: {t}");
}

pub fn foo() {
    dep_42::code();
    dep_42::code_inlined();
    dep_42::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
}
