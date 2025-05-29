pub fn code() {
    println!("Hello from dep_144");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_144");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_144: {t}");
}

pub fn foo() {
    dep_47::code();
    dep_47::code_inlined();
    dep_47::code_generic(1u32);
    dep_15::code();
    dep_15::code_inlined();
    dep_15::code_generic(1u32);
    dep_43::code();
    dep_43::code_inlined();
    dep_43::code_generic(1u32);
}
