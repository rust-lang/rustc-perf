pub fn code() {
    println!("Hello from dep_534");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_534");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_534: {t}");
}

pub fn foo() {
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_264::code();
    dep_264::code_inlined();
    dep_264::code_generic(1u32);
    dep_209::code();
    dep_209::code_inlined();
    dep_209::code_generic(1u32);
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
}
