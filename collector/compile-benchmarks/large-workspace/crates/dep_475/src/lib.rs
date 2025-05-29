pub fn code() {
    println!("Hello from dep_475");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_475");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_475: {t}");
}

pub fn foo() {
    dep_376::code();
    dep_376::code_inlined();
    dep_376::code_generic(1u32);
    dep_401::code();
    dep_401::code_inlined();
    dep_401::code_generic(1u32);
    dep_286::code();
    dep_286::code_inlined();
    dep_286::code_generic(1u32);
    dep_206::code();
    dep_206::code_inlined();
    dep_206::code_generic(1u32);
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
}
