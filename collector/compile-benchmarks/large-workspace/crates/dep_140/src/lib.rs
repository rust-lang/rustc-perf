pub fn code() {
    println!("Hello from dep_140");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_140");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_140: {t}");
}

pub fn foo() {
    dep_42::code();
    dep_42::code_inlined();
    dep_42::code_generic(1u32);
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
    dep_46::code();
    dep_46::code_inlined();
    dep_46::code_generic(1u32);
    dep_18::code();
    dep_18::code_inlined();
    dep_18::code_generic(1u32);
    dep_19::code();
    dep_19::code_inlined();
    dep_19::code_generic(1u32);
}
