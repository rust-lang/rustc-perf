pub fn code() {
    println!("Hello from dep_65");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_65");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_65: {t}");
}

pub fn foo() {
    dep_57::code();
    dep_57::code_inlined();
    dep_57::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_34::code();
    dep_34::code_inlined();
    dep_34::code_generic(1u32);
    dep_53::code();
    dep_53::code_inlined();
    dep_53::code_generic(1u32);
    dep_26::code();
    dep_26::code_inlined();
    dep_26::code_generic(1u32);
}
