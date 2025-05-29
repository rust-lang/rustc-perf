pub fn code() {
    println!("Hello from dep_412");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_412");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_412: {t}");
}

pub fn foo() {
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_262::code();
    dep_262::code_inlined();
    dep_262::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
}
