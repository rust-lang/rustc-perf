pub fn code() {
    println!("Hello from dep_63");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_63");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_63: {t}");
}

pub fn foo() {
    dep_14::code();
    dep_14::code_inlined();
    dep_14::code_generic(1u32);
    dep_34::code();
    dep_34::code_inlined();
    dep_34::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_50::code();
    dep_50::code_inlined();
    dep_50::code_generic(1u32);
}
