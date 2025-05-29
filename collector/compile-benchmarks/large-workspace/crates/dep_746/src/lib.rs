pub fn code() {
    println!("Hello from dep_746");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_746");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_746: {t}");
}

pub fn foo() {
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_265::code();
    dep_265::code_inlined();
    dep_265::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_199::code();
    dep_199::code_inlined();
    dep_199::code_generic(1u32);
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
}
