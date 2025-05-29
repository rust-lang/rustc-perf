pub fn code() {
    println!("Hello from dep_174");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_174");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_174: {t}");
}

pub fn foo() {
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
}
