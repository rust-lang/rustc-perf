pub fn code() {
    println!("Hello from dep_357");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_357");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_357: {t}");
}

pub fn foo() {
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
}
