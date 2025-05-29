pub fn code() {
    println!("Hello from dep_245");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_245");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_245: {t}");
}

pub fn foo() {
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
    dep_125::code();
    dep_125::code_inlined();
    dep_125::code_generic(1u32);
}
