pub fn code() {
    println!("Hello from dep_84");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_84");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_84: {t}");
}

pub fn foo() {
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
    dep_27::code();
    dep_27::code_inlined();
    dep_27::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_57::code();
    dep_57::code_inlined();
    dep_57::code_generic(1u32);
    dep_50::code();
    dep_50::code_inlined();
    dep_50::code_generic(1u32);
}
