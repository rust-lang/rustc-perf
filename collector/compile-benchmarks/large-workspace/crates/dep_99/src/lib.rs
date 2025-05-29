pub fn code() {
    println!("Hello from dep_99");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_99");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_99: {t}");
}

pub fn foo() {
    dep_43::code();
    dep_43::code_inlined();
    dep_43::code_generic(1u32);
    dep_57::code();
    dep_57::code_inlined();
    dep_57::code_generic(1u32);
    dep_12::code();
    dep_12::code_inlined();
    dep_12::code_generic(1u32);
    dep_28::code();
    dep_28::code_inlined();
    dep_28::code_generic(1u32);
    dep_51::code();
    dep_51::code_inlined();
    dep_51::code_generic(1u32);
}
