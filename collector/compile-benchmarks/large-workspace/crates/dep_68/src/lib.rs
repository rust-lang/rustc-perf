pub fn code() {
    println!("Hello from dep_68");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_68");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_68: {t}");
}

pub fn foo() {
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_51::code();
    dep_51::code_inlined();
    dep_51::code_generic(1u32);
    dep_14::code();
    dep_14::code_inlined();
    dep_14::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
    dep_19::code();
    dep_19::code_inlined();
    dep_19::code_generic(1u32);
}
