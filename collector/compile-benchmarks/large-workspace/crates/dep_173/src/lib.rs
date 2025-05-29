pub fn code() {
    println!("Hello from dep_173");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_173");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_173: {t}");
}

pub fn foo() {
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
    dep_65::code();
    dep_65::code_inlined();
    dep_65::code_generic(1u32);
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
}
