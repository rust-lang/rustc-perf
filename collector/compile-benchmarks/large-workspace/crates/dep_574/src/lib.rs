pub fn code() {
    println!("Hello from dep_574");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_574");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_574: {t}");
}

pub fn foo() {
    dep_372::code();
    dep_372::code_inlined();
    dep_372::code_generic(1u32);
    dep_222::code();
    dep_222::code_inlined();
    dep_222::code_generic(1u32);
    dep_358::code();
    dep_358::code_inlined();
    dep_358::code_generic(1u32);
    dep_321::code();
    dep_321::code_inlined();
    dep_321::code_generic(1u32);
}
