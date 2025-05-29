pub fn code() {
    println!("Hello from dep_122");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_122");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_122: {t}");
}

pub fn foo() {
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
    dep_27::code();
    dep_27::code_inlined();
    dep_27::code_generic(1u32);
    dep_55::code();
    dep_55::code_inlined();
    dep_55::code_generic(1u32);
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
}
