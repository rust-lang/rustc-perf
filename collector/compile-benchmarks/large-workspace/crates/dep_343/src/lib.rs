pub fn code() {
    println!("Hello from dep_343");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_343");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_343: {t}");
}

pub fn foo() {
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_65::code();
    dep_65::code_inlined();
    dep_65::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
}
