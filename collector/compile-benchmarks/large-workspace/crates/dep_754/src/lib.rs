pub fn code() {
    println!("Hello from dep_754");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_754");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_754: {t}");
}

pub fn foo() {
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_361::code();
    dep_361::code_inlined();
    dep_361::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_398::code();
    dep_398::code_inlined();
    dep_398::code_generic(1u32);
}
