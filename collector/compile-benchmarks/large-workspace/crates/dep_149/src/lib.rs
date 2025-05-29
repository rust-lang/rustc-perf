pub fn code() {
    println!("Hello from dep_149");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_149");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_149: {t}");
}

pub fn foo() {
    dep_22::code();
    dep_22::code_inlined();
    dep_22::code_generic(1u32);
    dep_34::code();
    dep_34::code_inlined();
    dep_34::code_generic(1u32);
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
    dep_40::code();
    dep_40::code_inlined();
    dep_40::code_generic(1u32);
}
