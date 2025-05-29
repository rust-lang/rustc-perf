pub fn code() {
    println!("Hello from dep_4");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_4");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_4: {t}");
}

