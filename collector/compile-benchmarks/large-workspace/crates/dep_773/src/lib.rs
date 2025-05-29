pub fn code() {
    println!("Hello from dep_773");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_773");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_773: {t}");
}

pub fn foo() {
    dep_251::code();
    dep_251::code_inlined();
    dep_251::code_generic(1u32);
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
}
