pub fn code() {
    println!("Hello from dep_440");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_440");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_440: {t}");
}

pub fn foo() {
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
    dep_406::code();
    dep_406::code_inlined();
    dep_406::code_generic(1u32);
    dep_375::code();
    dep_375::code_inlined();
    dep_375::code_generic(1u32);
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
}
