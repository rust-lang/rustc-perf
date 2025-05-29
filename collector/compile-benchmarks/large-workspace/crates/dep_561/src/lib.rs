pub fn code() {
    println!("Hello from dep_561");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_561");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_561: {t}");
}

pub fn foo() {
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_310::code();
    dep_310::code_inlined();
    dep_310::code_generic(1u32);
    dep_179::code();
    dep_179::code_inlined();
    dep_179::code_generic(1u32);
    dep_266::code();
    dep_266::code_inlined();
    dep_266::code_generic(1u32);
    dep_313::code();
    dep_313::code_inlined();
    dep_313::code_generic(1u32);
}
