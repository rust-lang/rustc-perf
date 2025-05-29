pub fn code() {
    println!("Hello from dep_154");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_154");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_154: {t}");
}

pub fn foo() {
    dep_20::code();
    dep_20::code_inlined();
    dep_20::code_generic(1u32);
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
    dep_35::code();
    dep_35::code_inlined();
    dep_35::code_generic(1u32);
    dep_46::code();
    dep_46::code_inlined();
    dep_46::code_generic(1u32);
    dep_24::code();
    dep_24::code_inlined();
    dep_24::code_generic(1u32);
}
