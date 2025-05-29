pub fn code() {
    println!("Hello from dep_124");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_124");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_124: {t}");
}

pub fn foo() {
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
    dep_34::code();
    dep_34::code_inlined();
    dep_34::code_generic(1u32);
    dep_42::code();
    dep_42::code_inlined();
    dep_42::code_generic(1u32);
    dep_29::code();
    dep_29::code_inlined();
    dep_29::code_generic(1u32);
    dep_47::code();
    dep_47::code_inlined();
    dep_47::code_generic(1u32);
}
