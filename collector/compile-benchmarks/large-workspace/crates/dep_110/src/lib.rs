pub fn code() {
    println!("Hello from dep_110");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_110");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_110: {t}");
}

pub fn foo() {
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
    dep_40::code();
    dep_40::code_inlined();
    dep_40::code_generic(1u32);
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
}
