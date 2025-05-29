pub fn code() {
    println!("Hello from dep_490");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_490");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_490: {t}");
}

pub fn foo() {
    dep_311::code();
    dep_311::code_inlined();
    dep_311::code_generic(1u32);
    dep_319::code();
    dep_319::code_inlined();
    dep_319::code_generic(1u32);
    dep_179::code();
    dep_179::code_inlined();
    dep_179::code_generic(1u32);
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
    dep_399::code();
    dep_399::code_inlined();
    dep_399::code_generic(1u32);
}
