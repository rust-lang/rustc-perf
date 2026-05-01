use gll::runtime::{Call, Continuation, ParseNodeKind, CodeLabel, ParseNodeShape, ParseNode, Range, traverse, nd::Arrow};
use std::any;
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug)]
pub enum ParseError<T> {
    TooShort(T),
    NoParse,
}

pub type ParseResult<'a, 'i, I, T> =
    Result<Handle<'a, 'i, I, T>, ParseError<Handle<'a, 'i, I, T>>>;

pub type Any = dyn any::Any;

#[derive(Debug)]
pub struct Ambiguity<T>(T);

pub struct Handle<'a, 'i: 'a, I: 'a + ::gll::runtime::Input, T: ?Sized> {
    pub node: ParseNode<'i, _P>,
    pub parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
    _marker: PhantomData<T>,
}

impl<'a, 'i, I: ::gll::runtime::Input, T: ?Sized> Copy for Handle<'a, 'i, I, T> {}

impl<'a, 'i, I: ::gll::runtime::Input, T: ?Sized> Clone for Handle<'a, 'i, I, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, 'i, I: ::gll::runtime::Input, T: ?Sized> Handle<'a, 'i, I, T> {
    pub fn source(self) -> &'a I::Slice {
        self.parser.input(self.node.range)
    }
    pub fn source_info(self) -> I::SourceInfo {
        self.parser.source_info(self.node.range)
    }
}

impl<'a, 'i, I: ::gll::runtime::Input, T> From<Ambiguity<Handle<'a, 'i, I, T>>> for Ambiguity<Handle<'a, 'i, I, Any>> {
    fn from(x: Ambiguity<Handle<'a, 'i, I, T>>) -> Self {
        Ambiguity(Handle {
            node: x.0.node,
            parser: x.0.parser,
            _marker: PhantomData,
        })
    }
}

impl<'a, 'i, I: ::gll::runtime::Input, T> From<Ambiguity<Handle<'a, 'i, I, [T]>>> for Ambiguity<Handle<'a, 'i, I, Any>> {
    fn from(x: Ambiguity<Handle<'a, 'i, I, [T]>>) -> Self {
        Ambiguity(Handle {
            node: x.0.node,
            parser: x.0.parser,
            _marker: PhantomData,
        })
    }
}

impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, ()> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.source_info())
    }
}

impl<'a, 'i, I: ::gll::runtime::Input, T> fmt::Debug for Handle<'a, 'i, I, [T]>
    where Handle<'a, 'i, I, T>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} => ", self.source_info())?;
        match self.all_list_heads() {
            ListHead::Cons(cons) => {
                for (i, (elem, rest)) in cons.enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    enum Elem<T, L> {
                        One(T),
                        Spread(L),
                    }
                    impl<T: fmt::Debug, L: fmt::Debug> fmt::Debug for Elem<T, L> {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            match self {
                                Elem::One(x) => fmt::Debug::fmt(x, f),
                                Elem::Spread(xs) => {
                                    write!(f, "..(")?;
                                    fmt::Debug::fmt(xs, f)?;
                                    write!(f, ")")
                                }
                            }
                        }
                    }
                    f.debug_list().entries(::std::iter::once(Elem::One(elem)).chain(rest.map(|r| {
                        match r {
                            Ok(x) => Elem::One(x),
                            Err(Ambiguity(xs)) => Elem::Spread(xs),
                        }
                    }))).finish()?;
                }
            }
            ListHead::Nil => {
                f.debug_list().entries(None::<()>).finish()?;
            }
        }
        Ok(())
    }
}

impl<'a, 'i, I: ::gll::runtime::Input, T> Iterator for Handle<'a, 'i, I, [T]> {
    type Item = Result<Handle<'a, 'i, I, T>, Ambiguity<Self>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.all_list_heads() {
            ListHead::Cons(mut iter) => {
                let (elem, rest) = iter.next().unwrap();
                let original = *self;
                *self = rest;
                if iter.next().is_none() {
                    Some(Ok(elem))
                } else {
                    match self.node.kind.shape() {
                        ParseNodeShape::Opt(_) => {
                            self.node.range = Range(original.node.range.split_at(0).0);
                        }
                        _ => unreachable!(),
                    }
                    match self.one_list_head() {
                        ListHead::Nil => {}
                        _ => unreachable!(),
                    }
                    Some(Err(Ambiguity(original)))
                }
            }
            ListHead::Nil => None,
        }
    }
}

pub enum ListHead<C> {
    Cons(C),
    Nil,
}

impl<'a, 'i, I: ::gll::runtime::Input, T> Handle<'a, 'i, I, [T]> {
    fn one_list_head(self) -> ListHead<Result<(Handle<'a, 'i, I, T>, Handle<'a, 'i, I, [T]>), Ambiguity<Self>>> {
        match self.all_list_heads() {
            ListHead::Cons(mut iter) => {
                let first = iter.next().unwrap();
                if iter.next().is_none() {
                    ListHead::Cons(Ok(first))
                } else {
                    ListHead::Cons(Err(Ambiguity(self)))
                }
            }
            ListHead::Nil => ListHead::Nil,
        }
    }
    fn all_list_heads(mut self) -> ListHead<impl Iterator<Item = (Handle<'a, 'i, I, T>, Handle<'a, 'i, I, [T]>)>> {
        if let ParseNodeShape::Opt(_) = self.node.kind.shape() {
            if let Some(opt_child) = self.node.unpack_opt() {
                self.node = opt_child;
            } else {
                return ListHead::Nil;
            }
        }
        ListHead::Cons(self.parser.sppf.all_splits(self.node).flat_map(move |(elem, rest)| {
            if let ParseNodeShape::Split(..) = rest.kind.shape() {
                Some(self.parser.sppf.all_splits(rest)).into_iter().flatten().chain(None)
            } else {
                None.into_iter().flatten().chain(Some((elem, rest)))
            }
        }).map(move |(elem, rest)| {
            (Handle {
                node: elem,
                parser: self.parser,
                _marker: PhantomData,
            }, Handle { node: rest, ..self })
        }))
    }
}
macro_rules! P {(&[::gll::proc_macro::FlatTokenPat::Delim('('),]) => (_P::_0);(&[::gll::proc_macro::FlatTokenPat::Delim(')'),]) => (_P::_1);(&[::gll::proc_macro::FlatTokenPat::Delim('['),]) => (_P::_2);(&[::gll::proc_macro::FlatTokenPat::Delim(']'),]) => (_P::_3);(&[::gll::proc_macro::FlatTokenPat::Delim('{'),]) => (_P::_4);(&[::gll::proc_macro::FlatTokenPat::Delim('}'),]) => (_P::_5);(&[::gll::proc_macro::FlatTokenPat::Ident(None),]) => (_P::_6);(&[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) => (_P::_7);(&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),]) => (_P::_8);(&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),]) => (_P::_9);(&[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]) => (_P::_10);(&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]) => (_P::_11);(&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),]) => (_P::_12);(&[::gll::proc_macro::FlatTokenPat::Literal,]) => (_P::_13);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) => (_P::_14);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },]) => (_P::_15);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) => (_P::_16);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) => (_P::_17);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) => (_P::_18);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) => (_P::_19);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) => (_P::_20);(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) => (_P::_21);((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => (_P::_22);((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*))) => (_P::_23);((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr)) => (_P::_24);((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => (_P::_25);((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*))) => (_P::_26);((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr)) => (_P::_27);((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*))) => (_P::_28);((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)) => (_P::_29);((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))) => (_P::_30);((&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) => (_P::_31);((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))) => (_P::_32);((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?))) => (_P::_33);((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => (_P::_34);((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) => (_P::_35);(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => (_P::_36);(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => (_P::_37);(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => (_P::_38);(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => (_P::_39);(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => (_P::_40);(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },])) => (_P::_41);(((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim('}'),])) => (_P::_42);(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)) => (_P::_43);(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))?)) => (_P::_44);((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => (_P::_45);((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => (_P::_46);((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr)) => (_P::_47);(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => (_P::_48);(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => (_P::_49);((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => (_P::_50);(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => (_P::_51);(((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => (_P::_52);((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => (_P::_53);((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => (_P::_54);((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),])) => (_P::_55);((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))) => (_P::_56);((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)) => (_P::_57);(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => (_P::_58);(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr)) => (_P::_59);(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)) => (_P::_60);(((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT)) => (_P::_61);(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])) => (_P::_62);(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },])) => (_P::_63);((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),])) => (_P::_64);((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),])) => (_P::_65);((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),])) => (_P::_66);((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])) => (_P::_67);((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])) => (_P::_68);((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => (_P::_69);((Expr*)) => (_P::_70);((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => (_P::_71);((Expr+ HACK)) => (_P::_72);((Expr?)) => (_P::_73);((LITERAL | ((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr) | (Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?)) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr) | ((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT) | (((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | ((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | &[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),] | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?)) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?)))) => (_P::_74);((TOKEN_TREE*)) => (_P::_75);((TOKEN_TREE+ HACK)) => (_P::_76);(Expr) => (_P::_77);(IDENT) => (_P::_78);(LIFETIME) => (_P::_79);(LITERAL) => (_P::_80);(ModuleContents) => (_P::_81);(PUNCT) => (_P::_82);(TOKEN_TREE) => (_P::_83);}#[allow(non_camel_case_types)]pub struct IDENT<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {_marker: PhantomData<(&'a (), &'i (), I)>,}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for IDENT<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {let mut d = f.debug_struct("IDENT");d.finish()}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, IDENT<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.source_info())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> IDENT<'a, 'i, I> {
            fn from_sppf(
                parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                _node: ParseNode<'i, _P>,
                _r: traverse!(typeof(ParseNode<'i, _P>) _),
            ) -> Self {IDENT{_marker: PhantomData,}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, IDENT<'a, 'i, I>> {pub fn one(self) -> Result<IDENT<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let r = traverse!(one(sppf, node) _);
                IDENT::from_sppf(self.parser, node, r)
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = IDENT<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();
            traverse!(all(sppf) _)
                .apply(node)
                .map(move |r| IDENT::from_sppf(self.parser, node, r))}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> IDENT<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, IDENT<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::IDENT,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(IDENT), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }#[allow(non_camel_case_types)]pub struct PUNCT<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {_marker: PhantomData<(&'a (), &'i (), I)>,}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for PUNCT<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {let mut d = f.debug_struct("PUNCT");d.finish()}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, PUNCT<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.source_info())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> PUNCT<'a, 'i, I> {
            fn from_sppf(
                parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                _node: ParseNode<'i, _P>,
                _r: traverse!(typeof(ParseNode<'i, _P>) _),
            ) -> Self {PUNCT{_marker: PhantomData,}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, PUNCT<'a, 'i, I>> {pub fn one(self) -> Result<PUNCT<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let r = traverse!(one(sppf, node) _);
                PUNCT::from_sppf(self.parser, node, r)
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = PUNCT<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();
            traverse!(all(sppf) _)
                .apply(node)
                .map(move |r| PUNCT::from_sppf(self.parser, node, r))}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> PUNCT<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, PUNCT<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::PUNCT,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(PUNCT), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }#[allow(non_camel_case_types)]pub struct LITERAL<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {_marker: PhantomData<(&'a (), &'i (), I)>,}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for LITERAL<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {let mut d = f.debug_struct("LITERAL");d.finish()}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, LITERAL<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.source_info())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> LITERAL<'a, 'i, I> {
            fn from_sppf(
                parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                _node: ParseNode<'i, _P>,
                _r: traverse!(typeof(ParseNode<'i, _P>) _),
            ) -> Self {LITERAL{_marker: PhantomData,}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, LITERAL<'a, 'i, I>> {pub fn one(self) -> Result<LITERAL<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let r = traverse!(one(sppf, node) _);
                LITERAL::from_sppf(self.parser, node, r)
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = LITERAL<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();
            traverse!(all(sppf) _)
                .apply(node)
                .map(move |r| LITERAL::from_sppf(self.parser, node, r))}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> LITERAL<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, LITERAL<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::LITERAL,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(LITERAL), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }#[allow(non_camel_case_types)]pub struct TOKEN_TREE<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {_marker: PhantomData<(&'a (), &'i (), I)>,}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for TOKEN_TREE<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {let mut d = f.debug_struct("TOKEN_TREE");d.finish()}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, TOKEN_TREE<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.source_info())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> TOKEN_TREE<'a, 'i, I> {
            fn from_sppf(
                parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                _node: ParseNode<'i, _P>,
                _r: traverse!(typeof(ParseNode<'i, _P>) { 0 _0: P!(&[::gll::proc_macro::FlatTokenPat::Ident(None),]) => ?,1 _1: P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) => ?,2 _2: P!(&[::gll::proc_macro::FlatTokenPat::Literal,]) => ?,3 _3: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ((?, ?), ?),4 _4: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ((?, ?), ?),5 _5: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim('}'),])) => ((?, ?), ?), }),
            ) -> Self {TOKEN_TREE{_marker: PhantomData,}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, TOKEN_TREE<'a, 'i, I>> {pub fn one(self) -> Result<TOKEN_TREE<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let r = traverse!(one(sppf, node) { 0 _0: P!(&[::gll::proc_macro::FlatTokenPat::Ident(None),]) => ?,1 _1: P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) => ?,2 _2: P!(&[::gll::proc_macro::FlatTokenPat::Literal,]) => ?,3 _3: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ((?, ?), ?),4 _4: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ((?, ?), ?),5 _5: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim('}'),])) => ((?, ?), ?), });
                TOKEN_TREE::from_sppf(self.parser, node, r)
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = TOKEN_TREE<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();
            traverse!(all(sppf) { 0 _0: P!(&[::gll::proc_macro::FlatTokenPat::Ident(None),]) => ?,1 _1: P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) => ?,2 _2: P!(&[::gll::proc_macro::FlatTokenPat::Literal,]) => ?,3 _3: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ((?, ?), ?),4 _4: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ((?, ?), ?),5 _5: P!(((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim('}'),])) => ((?, ?), ?), })
                .apply(node)
                .map(move |r| TOKEN_TREE::from_sppf(self.parser, node, r))}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> TOKEN_TREE<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, TOKEN_TREE<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::TOKEN_TREE,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(TOKEN_TREE), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }#[allow(non_camel_case_types)]pub struct LIFETIME<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {_marker: PhantomData<(&'a (), &'i (), I)>,}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for LIFETIME<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {let mut d = f.debug_struct("LIFETIME");d.finish()}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, LIFETIME<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.source_info())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> LIFETIME<'a, 'i, I> {
            fn from_sppf(
                parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                _node: ParseNode<'i, _P>,
                _r: traverse!(typeof(ParseNode<'i, _P>) _),
            ) -> Self {LIFETIME{_marker: PhantomData,}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, LIFETIME<'a, 'i, I>> {pub fn one(self) -> Result<LIFETIME<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let r = traverse!(one(sppf, node) _);
                LIFETIME::from_sppf(self.parser, node, r)
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = LIFETIME<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();
            traverse!(all(sppf) _)
                .apply(node)
                .map(move |r| LIFETIME::from_sppf(self.parser, node, r))}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> LIFETIME<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, LIFETIME<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::LIFETIME,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(LIFETIME), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }pub enum Expr<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {Literal(Handle<'a, 'i, I, LITERAL<'a, 'i, I>>),Paren {expr: Handle<'a, 'i, I, Expr<'a, 'i, I>>,},Borrow {expr: Handle<'a, 'i, I, Expr<'a, 'i, I>>,mutable: Option<Handle<'a, 'i, I, ()>>,},Box {expr: Handle<'a, 'i, I, Expr<'a, 'i, I>>,},Try {expr: Handle<'a, 'i, I, Expr<'a, 'i, I>>,},Range {start: Option<Handle<'a, 'i, I, Expr<'a, 'i, I>>>,end: Option<Handle<'a, 'i, I, Expr<'a, 'i, I>>>,},RangeInclusive {start: Option<Handle<'a, 'i, I, Expr<'a, 'i, I>>>,end: Handle<'a, 'i, I, Expr<'a, 'i, I>>,},Cast {expr: Handle<'a, 'i, I, Expr<'a, 'i, I>>,ty: Handle<'a, 'i, I, IDENT<'a, 'i, I>>,},Index {base: Handle<'a, 'i, I, Expr<'a, 'i, I>>,index: Handle<'a, 'i, I, Expr<'a, 'i, I>>,},Array {exprs: Handle<'a, 'i, I, [Expr<'a, 'i, I>]>,},Repeat {elem: Handle<'a, 'i, I, Expr<'a, 'i, I>>,count: Handle<'a, 'i, I, Expr<'a, 'i, I>>,},Tuple {exprs: Handle<'a, 'i, I, [Expr<'a, 'i, I>]>,},Call {callee: Handle<'a, 'i, I, Expr<'a, 'i, I>>,args: Handle<'a, 'i, I, [Expr<'a, 'i, I>]>,},MethodCall {args: Handle<'a, 'i, I, [Expr<'a, 'i, I>]>,receiver: Handle<'a, 'i, I, Expr<'a, 'i, I>>,method: Handle<'a, 'i, I, IDENT<'a, 'i, I>>,},Continue(Handle<'a, 'i, I, ()>),Break {value: Option<Handle<'a, 'i, I, Expr<'a, 'i, I>>>,},Return {value: Option<Handle<'a, 'i, I, Expr<'a, 'i, I>>>,},}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Expr<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {match self {Expr::Literal(x) => f.debug_tuple("Expr::Literal").field(x).finish(),Expr::Paren { expr: f_expr, } => {let mut d = f.debug_struct("Expr::Paren");d.field("expr", f_expr);d.finish()}Expr::Borrow { expr: f_expr, mutable: f_mutable, } => {let mut d = f.debug_struct("Expr::Borrow");d.field("expr", f_expr);if let Some(field) = f_mutable { d.field("mutable", field); }d.finish()}Expr::Box { expr: f_expr, } => {let mut d = f.debug_struct("Expr::Box");d.field("expr", f_expr);d.finish()}Expr::Try { expr: f_expr, } => {let mut d = f.debug_struct("Expr::Try");d.field("expr", f_expr);d.finish()}Expr::Range { start: f_start, end: f_end, } => {let mut d = f.debug_struct("Expr::Range");if let Some(field) = f_start { d.field("start", field); }if let Some(field) = f_end { d.field("end", field); }d.finish()}Expr::RangeInclusive { start: f_start, end: f_end, } => {let mut d = f.debug_struct("Expr::RangeInclusive");if let Some(field) = f_start { d.field("start", field); }d.field("end", f_end);d.finish()}Expr::Cast { expr: f_expr, ty: f_ty, } => {let mut d = f.debug_struct("Expr::Cast");d.field("expr", f_expr);d.field("ty", f_ty);d.finish()}Expr::Index { base: f_base, index: f_index, } => {let mut d = f.debug_struct("Expr::Index");d.field("base", f_base);d.field("index", f_index);d.finish()}Expr::Array { exprs: f_exprs, } => {let mut d = f.debug_struct("Expr::Array");d.field("exprs", f_exprs);d.finish()}Expr::Repeat { elem: f_elem, count: f_count, } => {let mut d = f.debug_struct("Expr::Repeat");d.field("elem", f_elem);d.field("count", f_count);d.finish()}Expr::Tuple { exprs: f_exprs, } => {let mut d = f.debug_struct("Expr::Tuple");d.field("exprs", f_exprs);d.finish()}Expr::Call { callee: f_callee, args: f_args, } => {let mut d = f.debug_struct("Expr::Call");d.field("callee", f_callee);d.field("args", f_args);d.finish()}Expr::MethodCall { args: f_args, receiver: f_receiver, method: f_method, } => {let mut d = f.debug_struct("Expr::MethodCall");d.field("args", f_args);d.field("receiver", f_receiver);d.field("method", f_method);d.finish()}Expr::Continue(x) => f.debug_tuple("Expr::Continue").field(x).finish(),Expr::Break { value: f_value, } => {let mut d = f.debug_struct("Expr::Break");if let Some(field) = f_value { d.field("value", field); }d.finish()}Expr::Return { value: f_value, } => {let mut d = f.debug_struct("Expr::Return");if let Some(field) = f_value { d.field("value", field); }d.finish()}}}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, Expr<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?} => ", self.source_info())?;
                    let mut first = true;
                    for x in self.all() {
                        if !first {
                            write!(f, " | ")?;
                        }
                        first = false;
                        fmt::Debug::fmt(&x, f)?;
                    }
                    Ok(())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> Expr<'a, 'i, I> {
                #[allow(non_snake_case)]
                fn Literal_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) _),
                ) -> Self {Expr::Literal(Handle { node: _node, parser, _marker: PhantomData, })}
                #[allow(non_snake_case)]
                fn Paren_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) ((_, _), _)),
                ) -> Self {Expr::Paren {expr: Handle { node: _r .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Borrow_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) ((_, [?]), _)),
                ) -> Self {Expr::Borrow {expr: Handle { node: _r .1, parser, _marker: PhantomData, },mutable: None.or(_r .0 .1 .0).map(|node| Handle {
                            node,
                            parser,
                            _marker: PhantomData,
                        }),}}
                #[allow(non_snake_case)]
                fn Box_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (_, _)),
                ) -> Self {Expr::Box {expr: Handle { node: _r .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Try_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (_, _)),
                ) -> Self {Expr::Try {expr: Handle { node: _r .0, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Range_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (([?], _), [?])),
                ) -> Self {Expr::Range {start: None.or(_r .0 .0 .0).map(|node| Handle {
                            node,
                            parser,
                            _marker: PhantomData,
                        }),end: None.or(_r .1 .0).map(|node| Handle {
                            node,
                            parser,
                            _marker: PhantomData,
                        }),}}
                #[allow(non_snake_case)]
                fn RangeInclusive_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (([?], _), _)),
                ) -> Self {Expr::RangeInclusive {start: None.or(_r .0 .0 .0).map(|node| Handle {
                            node,
                            parser,
                            _marker: PhantomData,
                        }),end: Handle { node: _r .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Cast_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) ((_, _), _)),
                ) -> Self {Expr::Cast {expr: Handle { node: _r .0 .0, parser, _marker: PhantomData, },ty: Handle { node: _r .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Index_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (((_, _), _), _)),
                ) -> Self {Expr::Index {base: Handle { node: _r .0 .0 .0, parser, _marker: PhantomData, },index: Handle { node: _r .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Array_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (((_, _), [?]), _)),
                ) -> Self {Expr::Array {exprs: Handle { node: _r .0 .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Repeat_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) ((((_, _), _), _), _)),
                ) -> Self {Expr::Repeat {elem: Handle { node: _r .0 .0 .0 .1, parser, _marker: PhantomData, },count: Handle { node: _r .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Tuple_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (((_, _), [?]), _)),
                ) -> Self {Expr::Tuple {exprs: Handle { node: _r .0 .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Call_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) ((((_, _), _), [?]), _)),
                ) -> Self {Expr::Call {callee: Handle { node: _r .0 .0 .0 .0, parser, _marker: PhantomData, },args: Handle { node: _r .0 .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn MethodCall_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) ((((((_, _), _), _), _), [?]), _)),
                ) -> Self {Expr::MethodCall {args: Handle { node: _r .0 .0 .1, parser, _marker: PhantomData, },receiver: Handle { node: _r .0 .0 .0 .0 .0 .0, parser, _marker: PhantomData, },method: Handle { node: _r .0 .0 .0 .0 .1, parser, _marker: PhantomData, },}}
                #[allow(non_snake_case)]
                fn Continue_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) _),
                ) -> Self {Expr::Continue(Handle { node: _node, parser, _marker: PhantomData, })}
                #[allow(non_snake_case)]
                fn Break_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (_, [?])),
                ) -> Self {Expr::Break {value: None.or(_r .1 .0).map(|node| Handle {
                            node,
                            parser,
                            _marker: PhantomData,
                        }),}}
                #[allow(non_snake_case)]
                fn Return_from_sppf(
                    parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                    _node: ParseNode<'i, _P>,
                    _r: traverse!(typeof(ParseNode<'i, _P>) (_, [?])),
                ) -> Self {Expr::Return {value: None.or(_r .1 .0).map(|node| Handle {
                            node,
                            parser,
                            _marker: PhantomData,
                        }),}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, Expr<'a, 'i, I>> {pub fn one(self) -> Result<Expr<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let node = sppf.one_choice(node)?;
                match node.kind {P!(LITERAL) => {
                    let r = traverse!(one(sppf, node) _);Expr::Literal_from_sppf(self.parser, node, r)
                    }P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => {
                    let r = traverse!(one(sppf, node) ((_, _), _));Expr::Paren_from_sppf(self.parser, node, r)
                    }P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)) => {
                    let r = traverse!(one(sppf, node) ((_, [?]), _));Expr::Borrow_from_sppf(self.parser, node, r)
                    }P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)) => {
                    let r = traverse!(one(sppf, node) (_, _));Expr::Box_from_sppf(self.parser, node, r)
                    }P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])) => {
                    let r = traverse!(one(sppf, node) (_, _));Expr::Try_from_sppf(self.parser, node, r)
                    }P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))) => {
                    let r = traverse!(one(sppf, node) (([?], _), [?]));Expr::Range_from_sppf(self.parser, node, r)
                    }P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)) => {
                    let r = traverse!(one(sppf, node) (([?], _), _));Expr::RangeInclusive_from_sppf(self.parser, node, r)
                    }P!(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)) => {
                    let r = traverse!(one(sppf, node) ((_, _), _));Expr::Cast_from_sppf(self.parser, node, r)
                    }P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => {
                    let r = traverse!(one(sppf, node) (((_, _), _), _));Expr::Index_from_sppf(self.parser, node, r)
                    }P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => {
                    let r = traverse!(one(sppf, node) (((_, _), [?]), _));Expr::Array_from_sppf(self.parser, node, r)
                    }P!(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => {
                    let r = traverse!(one(sppf, node) ((((_, _), _), _), _));Expr::Repeat_from_sppf(self.parser, node, r)
                    }P!((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => {
                    let r = traverse!(one(sppf, node) (((_, _), [?]), _));Expr::Tuple_from_sppf(self.parser, node, r)
                    }P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => {
                    let r = traverse!(one(sppf, node) ((((_, _), _), [?]), _));Expr::Call_from_sppf(self.parser, node, r)
                    }P!(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => {
                    let r = traverse!(one(sppf, node) ((((((_, _), _), _), _), [?]), _));Expr::MethodCall_from_sppf(self.parser, node, r)
                    }P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]) => {
                    let r = traverse!(one(sppf, node) _);Expr::Continue_from_sppf(self.parser, node, r)
                    }P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))) => {
                    let r = traverse!(one(sppf, node) (_, [?]));Expr::Break_from_sppf(self.parser, node, r)
                    }P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))) => {
                    let r = traverse!(one(sppf, node) (_, [?]));Expr::Return_from_sppf(self.parser, node, r)
                    }_ => unreachable!()}
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = Expr<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();#[derive(Clone)] enum Iter<_0,_1,_2,_3,_4,_5,_6,_7,_8,_9,_10,_11,_12,_13,_14,_15,_16,> {_0(_0),_1(_1),_2(_2),_3(_3),_4(_4),_5(_5),_6(_6),_7(_7),_8(_8),_9(_9),_10(_10),_11(_11),_12(_12),_13(_13),_14(_14),_15(_15),_16(_16),}impl<T, _0: Iterator<Item = T>,_1: Iterator<Item = T>,_2: Iterator<Item = T>,_3: Iterator<Item = T>,_4: Iterator<Item = T>,_5: Iterator<Item = T>,_6: Iterator<Item = T>,_7: Iterator<Item = T>,_8: Iterator<Item = T>,_9: Iterator<Item = T>,_10: Iterator<Item = T>,_11: Iterator<Item = T>,_12: Iterator<Item = T>,_13: Iterator<Item = T>,_14: Iterator<Item = T>,_15: Iterator<Item = T>,_16: Iterator<Item = T>,> Iterator for Iter<_0,_1,_2,_3,_4,_5,_6,_7,_8,_9,_10,_11,_12,_13,_14,_15,_16,>
        {
            type Item = T;
            fn next(&mut self) -> Option<T> {
                match self {
                    Iter::_0(iter) => iter.next(),
                    Iter::_1(iter) => iter.next(),
                    Iter::_2(iter) => iter.next(),
                    Iter::_3(iter) => iter.next(),
                    Iter::_4(iter) => iter.next(),
                    Iter::_5(iter) => iter.next(),
                    Iter::_6(iter) => iter.next(),
                    Iter::_7(iter) => iter.next(),
                    Iter::_8(iter) => iter.next(),
                    Iter::_9(iter) => iter.next(),
                    Iter::_10(iter) => iter.next(),
                    Iter::_11(iter) => iter.next(),
                    Iter::_12(iter) => iter.next(),
                    Iter::_13(iter) => iter.next(),
                    Iter::_14(iter) => iter.next(),
                    Iter::_15(iter) => iter.next(),
                    Iter::_16(iter) => iter.next(),
                }
            }
        }
        sppf.all_choices(node).flat_map(move |node| {
            match node.kind {
                P!(LITERAL) => Iter::_0(
                    traverse!(all(sppf) _)
                        .apply(node)
                        .map(move |r| Expr::Literal_from_sppf(self.parser, node, r))
                ),
                P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => Iter::_1(
                    traverse!(all(sppf) ((_, _), _))
                        .apply(node)
                        .map(move |r| Expr::Paren_from_sppf(self.parser, node, r))
                ),
                P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)) => Iter::_2(
                    traverse!(all(sppf) ((_, [?]), _))
                        .apply(node)
                        .map(move |r| Expr::Borrow_from_sppf(self.parser, node, r))
                ),
                P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)) => Iter::_3(
                    traverse!(all(sppf) (_, _))
                        .apply(node)
                        .map(move |r| Expr::Box_from_sppf(self.parser, node, r))
                ),
                P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])) => Iter::_4(
                    traverse!(all(sppf) (_, _))
                        .apply(node)
                        .map(move |r| Expr::Try_from_sppf(self.parser, node, r))
                ),
                P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))) => Iter::_5(
                    traverse!(all(sppf) (([?], _), [?]))
                        .apply(node)
                        .map(move |r| Expr::Range_from_sppf(self.parser, node, r))
                ),
                P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)) => Iter::_6(
                    traverse!(all(sppf) (([?], _), _))
                        .apply(node)
                        .map(move |r| Expr::RangeInclusive_from_sppf(self.parser, node, r))
                ),
                P!(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)) => Iter::_7(
                    traverse!(all(sppf) ((_, _), _))
                        .apply(node)
                        .map(move |r| Expr::Cast_from_sppf(self.parser, node, r))
                ),
                P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => Iter::_8(
                    traverse!(all(sppf) (((_, _), _), _))
                        .apply(node)
                        .map(move |r| Expr::Index_from_sppf(self.parser, node, r))
                ),
                P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => Iter::_9(
                    traverse!(all(sppf) (((_, _), [?]), _))
                        .apply(node)
                        .map(move |r| Expr::Array_from_sppf(self.parser, node, r))
                ),
                P!(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => Iter::_10(
                    traverse!(all(sppf) ((((_, _), _), _), _))
                        .apply(node)
                        .map(move |r| Expr::Repeat_from_sppf(self.parser, node, r))
                ),
                P!((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => Iter::_11(
                    traverse!(all(sppf) (((_, _), [?]), _))
                        .apply(node)
                        .map(move |r| Expr::Tuple_from_sppf(self.parser, node, r))
                ),
                P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => Iter::_12(
                    traverse!(all(sppf) ((((_, _), _), [?]), _))
                        .apply(node)
                        .map(move |r| Expr::Call_from_sppf(self.parser, node, r))
                ),
                P!(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => Iter::_13(
                    traverse!(all(sppf) ((((((_, _), _), _), _), [?]), _))
                        .apply(node)
                        .map(move |r| Expr::MethodCall_from_sppf(self.parser, node, r))
                ),
                P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]) => Iter::_14(
                    traverse!(all(sppf) _)
                        .apply(node)
                        .map(move |r| Expr::Continue_from_sppf(self.parser, node, r))
                ),
                P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))) => Iter::_15(
                    traverse!(all(sppf) (_, [?]))
                        .apply(node)
                        .map(move |r| Expr::Break_from_sppf(self.parser, node, r))
                ),
                P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))) => Iter::_16(
                    traverse!(all(sppf) (_, [?]))
                        .apply(node)
                        .map(move |r| Expr::Return_from_sppf(self.parser, node, r))
                ),
                _ => unreachable!(),
            }
        })}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> Expr<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, Expr<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::Expr,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(Expr), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }#[allow(non_camel_case_types)]pub struct ModuleContents<'a, 'i: 'a, I: 'a + ::gll::runtime::Input> {pub items: Handle<'a, 'i, I, [Expr<'a, 'i, I>]>,}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for ModuleContents<'a, 'i, I> {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {let mut d = f.debug_struct("ModuleContents");d.field("items", &self.items);d.finish()}}
            impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, ModuleContents<'a, 'i, I>> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?} => ", self.source_info())?;
                    let mut first = true;
                    for x in self.all() {
                        if !first {
                            write!(f, " | ")?;
                        }
                        first = false;
                        fmt::Debug::fmt(&x, f)?;
                    }
                    Ok(())
                }
            }
        impl<'a, 'i, I: ::gll::runtime::Input> ModuleContents<'a, 'i, I> {
            fn from_sppf(
                parser: &'a ::gll::runtime::Parser<'i, _P, _C, I>,
                _node: ParseNode<'i, _P>,
                _r: traverse!(typeof(ParseNode<'i, _P>) _),
            ) -> Self {ModuleContents{items: Handle { node: _r, parser, _marker: PhantomData, },}}}impl<'a, 'i, I: ::gll::runtime::Input> Handle<'a, 'i, I, ModuleContents<'a, 'i, I>> {pub fn one(self) -> Result<ModuleContents<'a, 'i, I>, Ambiguity<Self>> {
            // HACK(eddyb) using a closure to catch `Err`s from `?`
            (|| Ok({
                #[allow(unused_variables)]
                let sppf = &self.parser.sppf;
                let node = self.node.unpack_alias();
                let r = traverse!(one(sppf, node) _);
                ModuleContents::from_sppf(self.parser, node, r)
        }))().map_err(|::gll::runtime::MoreThanOne| Ambiguity(self))
        }pub fn all(self) -> impl Iterator<Item = ModuleContents<'a, 'i, I>> {
            #[allow(unused_variables)]
            let sppf = &self.parser.sppf;
            let node = self.node.unpack_alias();
            traverse!(all(sppf) _)
                .apply(node)
                .map(move |r| ModuleContents::from_sppf(self.parser, node, r))}}
    impl<'a, 'i, I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>> ModuleContents<'a, 'i, I> {
        pub fn parse_with<R>(
            input: I,
            f: impl for<'b, 'i2> FnOnce(
                &'b ::gll::runtime::Parser<'i2, _P, _C, I>,
                ParseResult<'b, 'i2, I, ModuleContents<'b, 'i2, I>>,
            ) -> R,
        ) -> R {
            ::gll::runtime::Parser::with(input, |mut parser, range| {
                let call = Call {
                    callee: _C::ModuleContents,
                    range,
                };
                parser.threads.spawn(
                    Continuation {
                        code: call.callee,
                        fn_input: call.range,
                        state: 0,
                    },
                    call.range,
                );
                parse(&mut parser);
                let result = parser.memoizer.longest_result(call);
                f(&parser, result.ok_or(ParseError::NoParse).and_then(|range| {
                    let handle = Handle {
                        node: ParseNode { kind: P!(ModuleContents), range },
                        parser: &parser,
                        _marker: PhantomData,
                    };
                    if range == call.range {
                        Ok(handle)
                    } else {
                        Err(ParseError::TooShort(handle))
                    }
                }))
            })
        }
    }
        fn parse<I>(p: &mut ::gll::runtime::Parser<_P, _C, I>)
        where I: ::gll::runtime::Input<Slice = [::gll::proc_macro::FlatToken]>
        {
            while let Some(Call { callee: mut c, range: _range }) = p.threads.steal() {
                match c.code {_C::IDENT => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(None),]) {
                    p.ret(c, _range);
                    }}_C::PUNCT => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) {
                    p.ret(c, _range);
                    }}_C::LITERAL => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Literal,]) {
                    p.ret(c, _range);
                    }}_C::TOKEN_TREE__0 => {
                p.ret(c, _range);}_C::TOKEN_TREE__1 => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) {
                    c.code = _C::TOKEN_TREE__0;
                    p.threads.spawn(c, _range);
                    }}_C::TOKEN_TREE__2__0 => {
                p.ret(c, _range);}_C::TOKEN_TREE__2__1 => {
                c.code = _C::TOKEN_TREE__2__0;
                p.call(Call { callee: _C::TOKEN_TREE__2, range: _range }, c);}_C::TOKEN_TREE__2 => {
                c.code = _C::TOKEN_TREE__2__1;
                p.call(Call { callee: _C::TOKEN_TREE, range: _range }, c);
                c.code = _C::TOKEN_TREE__2__0;
                p.threads.spawn(c, _range);}_C::TOKEN_TREE__3 => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) {
                    c.code = _C::TOKEN_TREE__0;
                    p.threads.spawn(c, _range);
                    }}_C::TOKEN_TREE__4__0 => {
                p.ret(c, _range);}_C::TOKEN_TREE__4__1 => {
                c.code = _C::TOKEN_TREE__4__0;
                p.call(Call { callee: _C::TOKEN_TREE__4, range: _range }, c);}_C::TOKEN_TREE__4 => {
                c.code = _C::TOKEN_TREE__4__1;
                p.call(Call { callee: _C::TOKEN_TREE, range: _range }, c);
                c.code = _C::TOKEN_TREE__4__0;
                p.threads.spawn(c, _range);}_C::TOKEN_TREE__5 => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('}'),]) {
                    c.code = _C::TOKEN_TREE__0;
                    p.threads.spawn(c, _range);
                    }}_C::TOKEN_TREE__6__0 => {
                p.ret(c, _range);}_C::TOKEN_TREE__6__1 => {
                c.code = _C::TOKEN_TREE__6__0;
                p.call(Call { callee: _C::TOKEN_TREE__6, range: _range }, c);}_C::TOKEN_TREE__6 => {
                c.code = _C::TOKEN_TREE__6__1;
                p.call(Call { callee: _C::TOKEN_TREE, range: _range }, c);
                c.code = _C::TOKEN_TREE__6__0;
                p.threads.spawn(c, _range);}_C::TOKEN_TREE => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(None),]) {
                    c.code = _C::TOKEN_TREE__0;
                    p.threads.spawn(c, _range);
                    }
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) {
                    c.code = _C::TOKEN_TREE__0;
                    p.threads.spawn(c, _range);
                    }
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Literal,]) {
                    c.code = _C::TOKEN_TREE__0;
                    p.threads.spawn(c, _range);
                    }
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('('),]) {
                    c.code = _C::TOKEN_TREE__1;
                    p.call(Call { callee: _C::TOKEN_TREE__2, range: _range }, c);
                    }
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('['),]) {
                    c.code = _C::TOKEN_TREE__3;
                    p.call(Call { callee: _C::TOKEN_TREE__4, range: _range }, c);
                    }
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('{'),]) {
                    c.code = _C::TOKEN_TREE__5;
                    p.call(Call { callee: _C::TOKEN_TREE__6, range: _range }, c);
                    }}_C::LIFETIME => {
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('\''), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Ident(None),]) {
                    p.ret(c, _range);
                    }}_C::Expr__0 => {
                p.sppf.add(P!((LITERAL | ((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr) | (Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?)) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr) | ((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT) | (((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | ((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | &[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),] | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?)) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?)))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__1__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr)), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) {
                    p.sppf.add(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__1 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('('),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__1__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    }}_C::Expr__2__0 => {
                p.sppf.add(P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__2__1 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                c.code = _C::Expr__2__0;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__2 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                        if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]) {
                        c.code = _C::Expr__2__1;
                        p.threads.spawn(c, _range);
                        }
                    c.code = _C::Expr__2__1;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__3__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__3 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__3__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    }}_C::Expr__4__0 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) {
                    p.sppf.add(P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__4 => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__4__0;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__5__0 => {
                p.sppf.add(P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__5__1 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) {
                    p.sppf.add(P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__5__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    c.code = _C::Expr__5__0;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__5 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__5__1;
                p.call(Call { callee: _C::Expr, range: _range }, c);
                c.code = _C::Expr__5__1;
                p.threads.spawn(c, _range);}_C::Expr__6__0 => {
                p.sppf.add(P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__6__1 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) {
                    p.sppf.add(P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__6__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    }}_C::Expr__6 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__6__1;
                p.call(Call { callee: _C::Expr, range: _range }, c);
                c.code = _C::Expr__6__1;
                p.threads.spawn(c, _range);}_C::Expr__7__0 => {
                p.sppf.add(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__7__1 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) {
                    p.sppf.add(P!((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__7__0;
                    p.call(Call { callee: _C::IDENT, range: _range }, c);
                    }}_C::Expr__7 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__7__1;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__8__0 => {
                p.sppf.add(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr)), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) {
                    p.sppf.add(P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__8__1 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('['),]) {
                    p.sppf.add(P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__8__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    }}_C::Expr__8 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__8__1;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__9__0 => {
                p.sppf.add(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) {
                    p.sppf.add(P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__9__1 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.code = _C::Expr__9__0;
                    p.threads.spawn(c, _range);
                    }
                c.code = _C::Expr__9__0;
                p.threads.spawn(c, _range);}_C::Expr__9__2__0 => {
                p.sppf.add(P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__9__2__1__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__9__2__1 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__9__2__1__0;
                    p.call(Call { callee: _C::Expr__9__2, range: _range }, c);
                    }}_C::Expr__9__2__2 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                c.code = _C::Expr__9__2__0;
                p.call(Call { callee: _C::Expr__9__2__1, range: _range }, c);
                c.code = _C::Expr__9__2__0;
                p.threads.spawn(c, _range);}_C::Expr__9__2 => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__9__2__2;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__9 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('['),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__9__1;
                    p.call(Call { callee: _C::Expr__9__2, range: _range }, c);
                    c.code = _C::Expr__9__1;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__10__0 => {
                p.sppf.add(P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr)), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) {
                    p.sppf.add(P!(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__10__1 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr)), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) {
                    p.sppf.add(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__10__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    }}_C::Expr__10 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('['),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__10__1;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    }}_C::Expr__11__0 => {
                p.sppf.add(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) {
                    p.sppf.add(P!((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__11__1 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.code = _C::Expr__11__0;
                    p.threads.spawn(c, _range);
                    }
                c.code = _C::Expr__11__0;
                p.threads.spawn(c, _range);}_C::Expr__11__2__0 => {
                p.sppf.add(P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__11__2__1__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__11__2__1 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__11__2__1__0;
                    p.call(Call { callee: _C::Expr__11__2, range: _range }, c);
                    }}_C::Expr__11__2__2 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                c.code = _C::Expr__11__2__0;
                p.call(Call { callee: _C::Expr__11__2__1, range: _range }, c);
                c.code = _C::Expr__11__2__0;
                p.threads.spawn(c, _range);}_C::Expr__11__2 => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__11__2__2;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__11 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('('),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__11__1;
                    p.call(Call { callee: _C::Expr__11__2, range: _range }, c);
                    c.code = _C::Expr__11__1;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__12__0 => {
                p.sppf.add(P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) {
                    p.sppf.add(P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__12__1 => {
                p.sppf.add(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.code = _C::Expr__12__0;
                    p.threads.spawn(c, _range);
                    }
                c.code = _C::Expr__12__0;
                p.threads.spawn(c, _range);}_C::Expr__12__2__0 => {
                p.sppf.add(P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__12__2__1__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__12__2__1 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__12__2__1__0;
                    p.call(Call { callee: _C::Expr__12__2, range: _range }, c);
                    }}_C::Expr__12__2__2 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                c.code = _C::Expr__12__2__0;
                p.call(Call { callee: _C::Expr__12__2__1, range: _range }, c);
                c.code = _C::Expr__12__2__0;
                p.threads.spawn(c, _range);}_C::Expr__12__2 => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__12__2__2;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__12__3 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('('),]) {
                    p.sppf.add(P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__12__1;
                    p.call(Call { callee: _C::Expr__12__2, range: _range }, c);
                    c.code = _C::Expr__12__1;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__12 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__12__3;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__13__0 => {
                p.sppf.add(P!((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) {
                    p.sppf.add(P!(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])), c.fn_input.subtract_suffix(_range), c.state);
                    p.ret(c, _range);
                    }}_C::Expr__13__1 => {
                p.sppf.add(P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.code = _C::Expr__13__0;
                    p.threads.spawn(c, _range);
                    }
                c.code = _C::Expr__13__0;
                p.threads.spawn(c, _range);}_C::Expr__13__2__0 => {
                p.sppf.add(P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__13__2__1__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__13__2__1 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__13__2__1__0;
                    p.call(Call { callee: _C::Expr__13__2, range: _range }, c);
                    }}_C::Expr__13__2__2 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                c.code = _C::Expr__13__2__0;
                p.call(Call { callee: _C::Expr__13__2__1, range: _range }, c);
                c.code = _C::Expr__13__2__0;
                p.threads.spawn(c, _range);}_C::Expr__13__2 => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__13__2__2;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__13__3 => {
                p.sppf.add(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT)), c.fn_input.subtract_suffix(_range), c.state);
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Delim('('),]) {
                    p.sppf.add(P!((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__13__1;
                    p.call(Call { callee: _C::Expr__13__2, range: _range }, c);
                    c.code = _C::Expr__13__1;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__13__4 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) {
                    p.sppf.add(P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])), c.fn_input.subtract_suffix(_range), c.state);
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__13__3;
                    p.call(Call { callee: _C::IDENT, range: _range }, c);
                    }}_C::Expr__13 => {
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::Expr__13__4;
                p.call(Call { callee: _C::Expr, range: _range }, c);}_C::Expr__14__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__14 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__14__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    c.code = _C::Expr__14__0;
                    p.threads.spawn(c, _range);
                    }}_C::Expr__15__0 => {
                p.sppf.add(P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))), c.fn_input.subtract_suffix(_range), c.state);
                p.ret(c, _range);}_C::Expr__15 => {
                assert_eq!(_range.start(), c.fn_input.start());
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),]) {
                    c.state = c.fn_input.subtract_suffix(_range).len();
                    c.code = _C::Expr__15__0;
                    p.call(Call { callee: _C::Expr, range: _range }, c);
                    c.code = _C::Expr__15__0;
                    p.threads.spawn(c, _range);
                    }}_C::Expr => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.state = P!(LITERAL).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::LITERAL, range: _range }, c);
                c.state = P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__1, range: _range }, c);
                c.state = P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__2, range: _range }, c);
                c.state = P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__3, range: _range }, c);
                c.state = P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__4, range: _range }, c);
                c.state = P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__5, range: _range }, c);
                c.state = P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__6, range: _range }, c);
                c.state = P!(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__7, range: _range }, c);
                c.state = P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__8, range: _range }, c);
                c.state = P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__9, range: _range }, c);
                c.state = P!(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__10, range: _range }, c);
                c.state = P!((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__11, range: _range }, c);
                c.state = P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__12, range: _range }, c);
                c.state = P!(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__13, range: _range }, c);
                c.state = P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]).to_usize();
                    if let Some(_range) = p.input_consume_left(_range, &[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]) {
                    c.code = _C::Expr__0;
                    p.threads.spawn(c, _range);
                    }
                c.state = P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__14, range: _range }, c);
                c.state = P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))).to_usize();
                c.code = _C::Expr__0;
                p.call(Call { callee: _C::Expr__15, range: _range }, c);}_C::ModuleContents__0 => {
                p.ret(c, _range);}_C::ModuleContents__1__0 => {
                p.ret(c, _range);}_C::ModuleContents__1__1 => {
                p.sppf.add(P!((Expr+ HACK)), c.fn_input.subtract_suffix(_range), c.state);
                c.code = _C::ModuleContents__1__0;
                p.threads.spawn(c, _range);}_C::ModuleContents__1__2 => {
                c.state = c.fn_input.subtract_suffix(_range).len();
                c.code = _C::ModuleContents__1__1;
                p.call(Call { callee: _C::ModuleContents__1, range: _range }, c);}_C::ModuleContents__1 => {
                assert_eq!(_range.start(), c.fn_input.start());
                c.code = _C::ModuleContents__1__2;
                p.call(Call { callee: _C::Expr, range: _range }, c);
                c.code = _C::ModuleContents__1__0;
                p.threads.spawn(c, _range);}_C::ModuleContents => {
                c.code = _C::ModuleContents__0;
                p.call(Call { callee: _C::ModuleContents__1, range: _range }, c);}} } }#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]pub enum _P { _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63, _64, _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78, _79, _80, _81, _82, _83,}impl fmt::Display for _P {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let s = match *self {P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),]) => "&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]",P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),]) => "&[::gll::proc_macro::FlatTokenPat::Delim(\')\'),]",P!(&[::gll::proc_macro::FlatTokenPat::Delim('['),]) => "&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),]",P!(&[::gll::proc_macro::FlatTokenPat::Delim(']'),]) => "&[::gll::proc_macro::FlatTokenPat::Delim(\']\'),]",P!(&[::gll::proc_macro::FlatTokenPat::Delim('{'),]) => "&[::gll::proc_macro::FlatTokenPat::Delim(\'{\'),]",P!(&[::gll::proc_macro::FlatTokenPat::Delim('}'),]) => "&[::gll::proc_macro::FlatTokenPat::Delim(\'}\'),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(None),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(None),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"as\")),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"box\")),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"break\")),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"continue\")),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"mut\")),]",P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),]) => "&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"return\")),]",P!(&[::gll::proc_macro::FlatTokenPat::Literal,]) => "&[::gll::proc_macro::FlatTokenPat::Literal,]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'&\'), joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'=\'), joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\';\'), joint: None },]",P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) => "&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'?\'), joint: None },]",P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]))",P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*))) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] (TOKEN_TREE*))",P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr)) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] Expr)",P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]))",P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*))) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] (TOKEN_TREE*))",P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr)) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] Expr)",P!((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*))) => "(&[::gll::proc_macro::FlatTokenPat::Delim(\'{\'),] (TOKEN_TREE*))",P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)) => "(&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"box\")),] Expr)",P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))) => "(&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"break\")),] (Expr?))",P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) => "(&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"mut\")),]?)",P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))) => "(&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"return\")),] (Expr?))",P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?))) => "(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'&\'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"mut\")),]?))",P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => "(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]))",P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) => "(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?))",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),])",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),])",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?))",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),])",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },])) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\';\'), joint: None },])",P!(((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim('}'),])) => "((&[::gll::proc_macro::FlatTokenPat::Delim(\'{\'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(\'}\'),])",P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)) => "((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'&\'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"mut\")),]?)) Expr)",P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))?)) => "((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]))?)",P!((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => "(((&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),])",P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => "(((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),])",P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr)) => "(((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\';\'), joint: None },]) Expr)",P!(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => "((((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\';\'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),])",P!(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => "((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),])",P!((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => "(((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?))",P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => "((((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),])",P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => "((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]))",P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => "(((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?))",P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => "(((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),])",P!((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),])) => "(((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),])",P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))) => "(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) (Expr?))",P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)) => "(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'=\'), joint: None },]) Expr)",P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => "((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]))",P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr)) => "((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),]) Expr)",P!(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)) => "((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some(\"as\")),]) IDENT)",P!(((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT)) => "((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) IDENT)",P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])) => "((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },])",P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },])) => "((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'=\'), joint: None },])",P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),])) => "(Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),])",P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),])) => "(Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),])",P!((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),])) => "(Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some(\"as\")),])",P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])) => "(Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },])",P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])) => "(Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'?\'), joint: None },])",P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => "(Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])",P!((Expr*)) => "(Expr*)",P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => "(Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])",P!((Expr+ HACK)) => "(Expr+ HACK)",P!((Expr?)) => "(Expr?)",P!((LITERAL | ((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr) | (Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?)) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr) | ((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT) | (((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | ((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | &[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),] | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?)) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?)))) => "(LITERAL | ((&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),]) | ((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'&\'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"mut\")),]?)) Expr) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"box\")),] Expr) | (Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'?\'), joint: None },]) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) (Expr?)) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'=\'), joint: None },]) Expr) | ((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some(\"as\")),]) IDENT) | (((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),]) | ((((&[::gll::proc_macro::FlatTokenPat::Delim(\'[\'),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\';\'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(\']\'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),]) | ((((Expr &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),]) | ((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\'.\'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim(\'(\'),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(\',\'), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(\')\'),]) | &[::gll::proc_macro::FlatTokenPat::Ident(Some(\"continue\")),] | (&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"break\")),] (Expr?)) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some(\"return\")),] (Expr?)))",P!((TOKEN_TREE*)) => "(TOKEN_TREE*)",P!((TOKEN_TREE+ HACK)) => "(TOKEN_TREE+ HACK)",P!(Expr) => "Expr",P!(IDENT) => "IDENT",P!(LIFETIME) => "LIFETIME",P!(LITERAL) => "LITERAL",P!(ModuleContents) => "ModuleContents",P!(PUNCT) => "PUNCT",P!(TOKEN_TREE) => "TOKEN_TREE",};write!(f, "{}", s)} }impl ParseNodeKind for _P {
        fn shape(self) -> ParseNodeShape<Self> {
            match self {P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Delim('['),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Delim(']'),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Delim('{'),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Delim('}'),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(None),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Literal,]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: None, joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) => ParseNodeShape::Opaque,P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) => ParseNodeShape::Opaque,P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),]), P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))),P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),]), P!((TOKEN_TREE*))),P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr)) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),]), P!(Expr)),P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('['),]), P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))),P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('['),]), P!((TOKEN_TREE*))),P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr)) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('['),]), P!(Expr)),P!((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Delim('{'),]), P!((TOKEN_TREE*))),P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr)) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),]), P!(Expr)),P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),]), P!((Expr?))),P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) => ParseNodeShape::Opt(P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),])),P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),]), P!((Expr?))),P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },]), P!((&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?))),P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => ParseNodeShape::Split(P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]), P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))),P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) => ParseNodeShape::Opt(P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (TOKEN_TREE*))), P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),])),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr)), P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),])),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (TOKEN_TREE*))), P!(&[::gll::proc_macro::FlatTokenPat::Delim(']'),])),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },])) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr)), P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },])),P!(((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*)) &[::gll::proc_macro::FlatTokenPat::Delim('}'),])) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Delim('{'),] (TOKEN_TREE*))), P!(&[::gll::proc_macro::FlatTokenPat::Delim('}'),])),P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr)) => ParseNodeShape::Split(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?))), P!(Expr)),P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))?)) => ParseNodeShape::Opt(P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])))),P!((((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ParseNodeShape::Split(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),])),P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ParseNodeShape::Split(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), P!(&[::gll::proc_macro::FlatTokenPat::Delim(']'),])),P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr)) => ParseNodeShape::Split(P!(((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },])), P!(Expr)),P!(((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ParseNodeShape::Split(P!((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr)), P!(&[::gll::proc_macro::FlatTokenPat::Delim(']'),])),P!(((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ParseNodeShape::Split(P!((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),])),P!((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => ParseNodeShape::Split(P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))),P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),])) => ParseNodeShape::Split(P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))), P!(&[::gll::proc_macro::FlatTokenPat::Delim(')'),])),P!(((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => ParseNodeShape::Split(P!((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),])), P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))),P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))) => ParseNodeShape::Split(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))), P!((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?))),P!((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),])) => ParseNodeShape::Split(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr)), P!(&[::gll::proc_macro::FlatTokenPat::Delim(']'),])),P!((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),])) => ParseNodeShape::Split(P!(((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT)), P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),])),P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?))) => ParseNodeShape::Split(P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])), P!((Expr?))),P!((((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr)) => ParseNodeShape::Split(P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },])), P!(Expr)),P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))) => ParseNodeShape::Split(P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),])), P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))),P!(((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr)) => ParseNodeShape::Split(P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),])), P!(Expr)),P!(((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT)) => ParseNodeShape::Split(P!((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),])), P!(IDENT)),P!(((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT)) => ParseNodeShape::Split(P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])), P!(IDENT)),P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])) => ParseNodeShape::Split(P!((Expr?)), P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])),P!(((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },])) => ParseNodeShape::Split(P!((Expr?)), P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },])),P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),])) => ParseNodeShape::Split(P!(Expr), P!(&[::gll::proc_macro::FlatTokenPat::Delim('('),])),P!((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),])) => ParseNodeShape::Split(P!(Expr), P!(&[::gll::proc_macro::FlatTokenPat::Delim('['),])),P!((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),])) => ParseNodeShape::Split(P!(Expr), P!(&[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),])),P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])) => ParseNodeShape::Split(P!(Expr), P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },])),P!((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])) => ParseNodeShape::Split(P!(Expr), P!(&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },])),P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => ParseNodeShape::Opt(P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))),P!((Expr*)) => ParseNodeShape::Opt(P!((Expr+ HACK))),P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => ParseNodeShape::Split(P!(Expr), P!(((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },] (Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]))?))),P!((Expr+ HACK)) => ParseNodeShape::Split(P!(Expr), P!((Expr*))),P!((Expr?)) => ParseNodeShape::Opt(P!(Expr)),P!((LITERAL | ((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr) | (Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?)) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr) | ((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT) | (((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | ((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | &[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),] | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?)) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?)))) => ParseNodeShape::Choice,P!((TOKEN_TREE*)) => ParseNodeShape::Opt(P!((TOKEN_TREE+ HACK))),P!((TOKEN_TREE+ HACK)) => ParseNodeShape::Split(P!(TOKEN_TREE), P!((TOKEN_TREE*))),P!(Expr) => ParseNodeShape::Alias(P!((LITERAL | ((&[::gll::proc_macro::FlatTokenPat::Delim('('),] Expr) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('&'), joint: None },] (&[::gll::proc_macro::FlatTokenPat::Ident(Some("mut")),]?)) Expr) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("box")),] Expr) | (Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('?'), joint: None },]) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) (Expr?)) | (((Expr?) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: Some(true) },::gll::proc_macro::FlatTokenPat::Punct { ch: Some('='), joint: None },]) Expr) | ((Expr &[::gll::proc_macro::FlatTokenPat::Ident(Some("as")),]) IDENT) | (((Expr &[::gll::proc_macro::FlatTokenPat::Delim('['),]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('['),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | ((((&[::gll::proc_macro::FlatTokenPat::Delim('['),] Expr) &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(';'), joint: None },]) Expr) &[::gll::proc_macro::FlatTokenPat::Delim(']'),]) | (((&[::gll::proc_macro::FlatTokenPat::Delim('('),] (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((Expr &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | ((((((Expr &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some('.'), joint: None },]) IDENT) &[::gll::proc_macro::FlatTokenPat::Delim('('),]) (Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) (&[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },]?)) &[::gll::proc_macro::FlatTokenPat::Delim(')'),]) | &[::gll::proc_macro::FlatTokenPat::Ident(Some("continue")),] | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("break")),] (Expr?)) | (&[::gll::proc_macro::FlatTokenPat::Ident(Some("return")),] (Expr?))))),P!(IDENT) => ParseNodeShape::Opaque,P!(LIFETIME) => ParseNodeShape::Opaque,P!(LITERAL) => ParseNodeShape::Opaque,P!(ModuleContents) => ParseNodeShape::Alias(P!((Expr*))),P!(PUNCT) => ParseNodeShape::Opaque,P!(TOKEN_TREE) => ParseNodeShape::Opaque,} }fn from_usize(i: usize) -> Self {
        match i {
        0 => _P::_0,
        1 => _P::_1,
        2 => _P::_2,
        3 => _P::_3,
        4 => _P::_4,
        5 => _P::_5,
        6 => _P::_6,
        7 => _P::_7,
        8 => _P::_8,
        9 => _P::_9,
        10 => _P::_10,
        11 => _P::_11,
        12 => _P::_12,
        13 => _P::_13,
        14 => _P::_14,
        15 => _P::_15,
        16 => _P::_16,
        17 => _P::_17,
        18 => _P::_18,
        19 => _P::_19,
        20 => _P::_20,
        21 => _P::_21,
        22 => _P::_22,
        23 => _P::_23,
        24 => _P::_24,
        25 => _P::_25,
        26 => _P::_26,
        27 => _P::_27,
        28 => _P::_28,
        29 => _P::_29,
        30 => _P::_30,
        31 => _P::_31,
        32 => _P::_32,
        33 => _P::_33,
        34 => _P::_34,
        35 => _P::_35,
        36 => _P::_36,
        37 => _P::_37,
        38 => _P::_38,
        39 => _P::_39,
        40 => _P::_40,
        41 => _P::_41,
        42 => _P::_42,
        43 => _P::_43,
        44 => _P::_44,
        45 => _P::_45,
        46 => _P::_46,
        47 => _P::_47,
        48 => _P::_48,
        49 => _P::_49,
        50 => _P::_50,
        51 => _P::_51,
        52 => _P::_52,
        53 => _P::_53,
        54 => _P::_54,
        55 => _P::_55,
        56 => _P::_56,
        57 => _P::_57,
        58 => _P::_58,
        59 => _P::_59,
        60 => _P::_60,
        61 => _P::_61,
        62 => _P::_62,
        63 => _P::_63,
        64 => _P::_64,
        65 => _P::_65,
        66 => _P::_66,
        67 => _P::_67,
        68 => _P::_68,
        69 => _P::_69,
        70 => _P::_70,
        71 => _P::_71,
        72 => _P::_72,
        73 => _P::_73,
        74 => _P::_74,
        75 => _P::_75,
        76 => _P::_76,
        77 => _P::_77,
        78 => _P::_78,
        79 => _P::_79,
        80 => _P::_80,
        81 => _P::_81,
        82 => _P::_82,
        83 => _P::_83,_ => unreachable!(),} }fn to_usize(self) -> usize { self as usize }}impl<'a, 'i, I: ::gll::runtime::Input> fmt::Debug for Handle<'a, 'i, I, Any> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.node.kind {
        P!((Expr* % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => write!(f, "{:?}", Handle::<_, [Expr<_>]> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!((Expr*)) => write!(f, "{:?}", Handle::<_, [Expr<_>]> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!((Expr+ % &[::gll::proc_macro::FlatTokenPat::Punct { ch: Some(','), joint: None },])) => write!(f, "{:?}", Handle::<_, [Expr<_>]> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!((Expr+ HACK)) => write!(f, "{:?}", Handle::<_, [Expr<_>]> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!((TOKEN_TREE*)) => write!(f, "{:?}", Handle::<_, [TOKEN_TREE<_>]> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!((TOKEN_TREE+ HACK)) => write!(f, "{:?}", Handle::<_, [TOKEN_TREE<_>]> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(Expr) => write!(f, "{:?}", Handle::<_, Expr<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(IDENT) => write!(f, "{:?}", Handle::<_, IDENT<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(LIFETIME) => write!(f, "{:?}", Handle::<_, LIFETIME<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(LITERAL) => write!(f, "{:?}", Handle::<_, LITERAL<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(ModuleContents) => write!(f, "{:?}", Handle::<_, ModuleContents<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(PUNCT) => write!(f, "{:?}", Handle::<_, PUNCT<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        P!(TOKEN_TREE) => write!(f, "{:?}", Handle::<_, TOKEN_TREE<_>> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),
        _ => write!(f, "{:?}", Handle::<_, ()> {
            node: self.node,
            parser: self.parser,
            _marker: PhantomData,
        }),} } }
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum _C {IDENT,PUNCT,LITERAL,TOKEN_TREE,LIFETIME,Expr,ModuleContents,TOKEN_TREE__0,TOKEN_TREE__1,TOKEN_TREE__2,TOKEN_TREE__3,TOKEN_TREE__4,TOKEN_TREE__5,TOKEN_TREE__6,TOKEN_TREE__2__0,TOKEN_TREE__2__1,TOKEN_TREE__4__0,TOKEN_TREE__4__1,TOKEN_TREE__6__0,TOKEN_TREE__6__1,Expr__0,Expr__1,Expr__2,Expr__3,Expr__4,Expr__5,Expr__6,Expr__7,Expr__8,Expr__9,Expr__10,Expr__11,Expr__12,Expr__13,Expr__14,Expr__15,Expr__1__0,Expr__2__0,Expr__2__1,Expr__3__0,Expr__4__0,Expr__5__0,Expr__5__1,Expr__6__0,Expr__6__1,Expr__7__0,Expr__7__1,Expr__8__0,Expr__8__1,Expr__9__0,Expr__9__1,Expr__9__2,Expr__9__2__0,Expr__9__2__1,Expr__9__2__2,Expr__9__2__1__0,Expr__10__0,Expr__10__1,Expr__11__0,Expr__11__1,Expr__11__2,Expr__11__2__0,Expr__11__2__1,Expr__11__2__2,Expr__11__2__1__0,Expr__12__0,Expr__12__1,Expr__12__2,Expr__12__3,Expr__12__2__0,Expr__12__2__1,Expr__12__2__2,Expr__12__2__1__0,Expr__13__0,Expr__13__1,Expr__13__2,Expr__13__3,Expr__13__4,Expr__13__2__0,Expr__13__2__1,Expr__13__2__2,Expr__13__2__1__0,Expr__14__0,Expr__15__0,ModuleContents__0,ModuleContents__1,ModuleContents__1__0,ModuleContents__1__1,ModuleContents__1__2,}impl CodeLabel for _C {
        fn enclosing_fn(self) -> Self {
            match self {_C::IDENT => _C::IDENT,_C::PUNCT => _C::PUNCT,_C::LITERAL => _C::LITERAL,_C::TOKEN_TREE => _C::TOKEN_TREE,_C::LIFETIME => _C::LIFETIME,_C::Expr => _C::Expr,_C::ModuleContents => _C::ModuleContents,_C::TOKEN_TREE__0 => _C::TOKEN_TREE,_C::TOKEN_TREE__1 => _C::TOKEN_TREE,_C::TOKEN_TREE__2 => _C::TOKEN_TREE__2,_C::TOKEN_TREE__3 => _C::TOKEN_TREE,_C::TOKEN_TREE__4 => _C::TOKEN_TREE__4,_C::TOKEN_TREE__5 => _C::TOKEN_TREE,_C::TOKEN_TREE__6 => _C::TOKEN_TREE__6,_C::TOKEN_TREE__2__0 => _C::TOKEN_TREE__2,_C::TOKEN_TREE__2__1 => _C::TOKEN_TREE__2,_C::TOKEN_TREE__4__0 => _C::TOKEN_TREE__4,_C::TOKEN_TREE__4__1 => _C::TOKEN_TREE__4,_C::TOKEN_TREE__6__0 => _C::TOKEN_TREE__6,_C::TOKEN_TREE__6__1 => _C::TOKEN_TREE__6,_C::Expr__0 => _C::Expr,_C::Expr__1 => _C::Expr__1,_C::Expr__2 => _C::Expr__2,_C::Expr__3 => _C::Expr__3,_C::Expr__4 => _C::Expr__4,_C::Expr__5 => _C::Expr__5,_C::Expr__6 => _C::Expr__6,_C::Expr__7 => _C::Expr__7,_C::Expr__8 => _C::Expr__8,_C::Expr__9 => _C::Expr__9,_C::Expr__10 => _C::Expr__10,_C::Expr__11 => _C::Expr__11,_C::Expr__12 => _C::Expr__12,_C::Expr__13 => _C::Expr__13,_C::Expr__14 => _C::Expr__14,_C::Expr__15 => _C::Expr__15,_C::Expr__1__0 => _C::Expr__1,_C::Expr__2__0 => _C::Expr__2,_C::Expr__2__1 => _C::Expr__2,_C::Expr__3__0 => _C::Expr__3,_C::Expr__4__0 => _C::Expr__4,_C::Expr__5__0 => _C::Expr__5,_C::Expr__5__1 => _C::Expr__5,_C::Expr__6__0 => _C::Expr__6,_C::Expr__6__1 => _C::Expr__6,_C::Expr__7__0 => _C::Expr__7,_C::Expr__7__1 => _C::Expr__7,_C::Expr__8__0 => _C::Expr__8,_C::Expr__8__1 => _C::Expr__8,_C::Expr__9__0 => _C::Expr__9,_C::Expr__9__1 => _C::Expr__9,_C::Expr__9__2 => _C::Expr__9__2,_C::Expr__9__2__0 => _C::Expr__9__2,_C::Expr__9__2__1 => _C::Expr__9__2__1,_C::Expr__9__2__2 => _C::Expr__9__2,_C::Expr__9__2__1__0 => _C::Expr__9__2__1,_C::Expr__10__0 => _C::Expr__10,_C::Expr__10__1 => _C::Expr__10,_C::Expr__11__0 => _C::Expr__11,_C::Expr__11__1 => _C::Expr__11,_C::Expr__11__2 => _C::Expr__11__2,_C::Expr__11__2__0 => _C::Expr__11__2,_C::Expr__11__2__1 => _C::Expr__11__2__1,_C::Expr__11__2__2 => _C::Expr__11__2,_C::Expr__11__2__1__0 => _C::Expr__11__2__1,_C::Expr__12__0 => _C::Expr__12,_C::Expr__12__1 => _C::Expr__12,_C::Expr__12__2 => _C::Expr__12__2,_C::Expr__12__3 => _C::Expr__12,_C::Expr__12__2__0 => _C::Expr__12__2,_C::Expr__12__2__1 => _C::Expr__12__2__1,_C::Expr__12__2__2 => _C::Expr__12__2,_C::Expr__12__2__1__0 => _C::Expr__12__2__1,_C::Expr__13__0 => _C::Expr__13,_C::Expr__13__1 => _C::Expr__13,_C::Expr__13__2 => _C::Expr__13__2,_C::Expr__13__3 => _C::Expr__13,_C::Expr__13__4 => _C::Expr__13,_C::Expr__13__2__0 => _C::Expr__13__2,_C::Expr__13__2__1 => _C::Expr__13__2__1,_C::Expr__13__2__2 => _C::Expr__13__2,_C::Expr__13__2__1__0 => _C::Expr__13__2__1,_C::Expr__14__0 => _C::Expr__14,_C::Expr__15__0 => _C::Expr__15,_C::ModuleContents__0 => _C::ModuleContents,_C::ModuleContents__1 => _C::ModuleContents__1,_C::ModuleContents__1__0 => _C::ModuleContents__1,_C::ModuleContents__1__1 => _C::ModuleContents__1,_C::ModuleContents__1__2 => _C::ModuleContents__1,} } }