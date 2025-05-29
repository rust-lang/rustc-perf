pub fn code() {
    println!("Hello from dep_96");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_96");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_96: {t}");
}

pub fn foo() {
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_16::code();
    dep_16::code_inlined();
    dep_16::code_generic(1u32);
    dep_41::code();
    dep_41::code_inlined();
    dep_41::code_generic(1u32);
}
