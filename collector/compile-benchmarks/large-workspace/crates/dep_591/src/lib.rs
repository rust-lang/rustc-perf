pub fn code() {
    println!("Hello from dep_591");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_591");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_591: {t}");
}

pub fn foo() {
    dep_403::code();
    dep_403::code_inlined();
    dep_403::code_generic(1u32);
    dep_301::code();
    dep_301::code_inlined();
    dep_301::code_generic(1u32);
    dep_401::code();
    dep_401::code_inlined();
    dep_401::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_331::code();
    dep_331::code_inlined();
    dep_331::code_generic(1u32);
}
