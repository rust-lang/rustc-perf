pub fn code() {
    println!("Hello from dep_726");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_726");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_726: {t}");
}

pub fn foo() {
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
    dep_177::code();
    dep_177::code_inlined();
    dep_177::code_generic(1u32);
}
