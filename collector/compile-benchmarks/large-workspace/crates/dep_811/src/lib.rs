pub fn code() {
    println!("Hello from dep_811");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_811");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_811: {t}");
}

pub fn foo() {
    dep_316::code();
    dep_316::code_inlined();
    dep_316::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
    dep_200::code();
    dep_200::code_inlined();
    dep_200::code_generic(1u32);
}
