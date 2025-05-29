pub fn code() {
    println!("Hello from dep_289");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_289");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_289: {t}");
}

pub fn foo() {
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
    dep_80::code();
    dep_80::code_inlined();
    dep_80::code_generic(1u32);
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
}
