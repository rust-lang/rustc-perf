pub fn code() {
    println!("Hello from dep_273");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_273");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_273: {t}");
}

pub fn foo() {
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_118::code();
    dep_118::code_inlined();
    dep_118::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
}
