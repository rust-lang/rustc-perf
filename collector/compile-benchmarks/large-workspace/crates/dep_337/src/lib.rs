pub fn code() {
    println!("Hello from dep_337");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_337");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_337: {t}");
}

pub fn foo() {
    dep_105::code();
    dep_105::code_inlined();
    dep_105::code_generic(1u32);
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
}
