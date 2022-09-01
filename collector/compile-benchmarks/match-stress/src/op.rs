use tokens;

ast_enum! {
    pub enum BinOp {
        /// The `+` operator (addition)
        Add(tokens::Add),
        /// The `-` operator (subtraction)
        Sub(tokens::Sub),
        /// The `*` operator (multiplication)
        Mul(tokens::Star),
        /// The `/` operator (division)
        Div(tokens::Div),
        /// The `%` operator (modulus)
        Rem(tokens::Rem),
        /// The `&&` operator (logical and)
        And(tokens::AndAnd),
        /// The `||` operator (logical or)
        Or(tokens::OrOr),
        /// The `^` operator (bitwise xor)
        BitXor(tokens::Caret),
        /// The `&` operator (bitwise and)
        BitAnd(tokens::And),
        /// The `|` operator (bitwise or)
        BitOr(tokens::Or),
        /// The `<<` operator (shift left)
        Shl(tokens::Shl),
        /// The `>>` operator (shift right)
        Shr(tokens::Shr),
        /// The `==` operator (equality)
        Eq(tokens::EqEq),
        /// The `<` operator (less than)
        Lt(tokens::Lt),
        /// The `<=` operator (less than or equal to)
        Le(tokens::Le),
        /// The `!=` operator (not equal to)
        Ne(tokens::Ne),
        /// The `>=` operator (greater than or equal to)
        Ge(tokens::Ge),
        /// The `>` operator (greater than)
        Gt(tokens::Gt),
        /// The `+=` operator
        AddEq(tokens::AddEq),
        /// The `-=` operator
        SubEq(tokens::SubEq),
        /// The `*=` operator
        MulEq(tokens::MulEq),
        /// The `/=` operator
        DivEq(tokens::DivEq),
        /// The `%=` operator
        RemEq(tokens::RemEq),
        /// The `^=` operator
        BitXorEq(tokens::CaretEq),
        /// The `&=` operator
        BitAndEq(tokens::AndEq),
        /// The `|=` operator
        BitOrEq(tokens::OrEq),
        /// The `<<=` operator
        ShlEq(tokens::ShlEq),
        /// The `>>=` operator
        ShrEq(tokens::ShrEq),
    }
}

ast_enum! {
    pub enum UnOp {
        /// The `*` operator for dereferencing
        Deref(tokens::Star),
        /// The `!` operator for logical inversion
        Not(tokens::Bang),
        /// The `-` operator for negation
        Neg(tokens::Sub),
    }
}
