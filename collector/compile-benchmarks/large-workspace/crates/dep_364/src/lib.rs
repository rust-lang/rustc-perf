pub fn code() {
    println!("Hello from dep_364");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_364");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_364: {t}");
}

pub fn foo() {
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_106::code();
    dep_106::code_inlined();
    dep_106::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
}
