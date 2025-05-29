pub fn code() {
    println!("Hello from dep_147");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_147");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_147: {t}");
}

pub fn foo() {
    dep_47::code();
    dep_47::code_inlined();
    dep_47::code_generic(1u32);
    dep_40::code();
    dep_40::code_inlined();
    dep_40::code_generic(1u32);
}
