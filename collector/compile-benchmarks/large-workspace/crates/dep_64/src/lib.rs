pub fn code() {
    println!("Hello from dep_64");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_64");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_64: {t}");
}

pub fn foo() {
    dep_41::code();
    dep_41::code_inlined();
    dep_41::code_generic(1u32);
    dep_42::code();
    dep_42::code_inlined();
    dep_42::code_generic(1u32);
    dep_12::code();
    dep_12::code_inlined();
    dep_12::code_generic(1u32);
    dep_45::code();
    dep_45::code_inlined();
    dep_45::code_generic(1u32);
}
