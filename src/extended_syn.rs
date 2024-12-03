//! Extra helpers missing from the [`syn`] crate.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    AngleBracketedGenericArguments, Attribute, Expr, ExprCall, ExprMethodCall, Ident, Token,
};

/// Holds extra tokens.
pub(crate) mod token {
    macro_rules! custom_keyword {
        ($i:ident as $n:ident) => {
            paste::paste! {
                pub(crate) use [<__private_ $i:lower>]::$i as $n;

                mod [<__private_ $i:lower>] {
                    syn::custom_keyword!($i);
                }
            }
        };
    }

    custom_keyword!(Ok as KwOk);
    custom_keyword!(Some as KwSome);
    custom_keyword!(Cond as KwCond);
    custom_keyword!(CondClone as KwCondClone);
    custom_keyword!(first as KwFirst);
    custom_keyword!(last as KwLast);
}

/// Parse a `(T)`.
pub(crate) fn parse_required_parens<T: Parse>(input: ParseStream) -> syn::Result<T> {
    let content;
    parenthesized!(content in input);
    content.parse()
}

/// Parse a `(T)` or `T`.
pub(crate) fn parse_optional_parens<T: Parse>(input: ParseStream) -> syn::Result<T> {
    if input.peek(Paren) {
        parse_required_parens(input)
    } else {
        input.parse()
    }
}

/// Parse a `(T)` or `T` or only `(T)` based on given flag.
pub(crate) fn parse_parens<T: Parse>(input: ParseStream, required: bool) -> syn::Result<T> {
    if required {
        parse_required_parens(input)
    } else {
        parse_optional_parens(input)
    }
}

pub(crate) fn attrs_to_tokens(attrs: &Vec<Attribute>) -> TokenStream2 {
    attrs.iter().map(|i| i.into_token_stream()).collect()
}

pub(crate) fn expr_args_to_token_args(
    args: &Punctuated<Expr, Token![,]>,
) -> Punctuated<TokenStream2, Token![,]> {
    args.iter()
        .map(|i| i.into_token_stream())
        .collect::<Punctuated<TokenStream2, Token![,]>>()
}

/// Split a callable expression by arguments and body.
pub(crate) trait SplitArgs {
    /// Return arguments as tokens and arguments in a separate value.
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>);
}

macro_rules! default_split_args {
    ($i:path$(,)?) => {
        impl SplitArgs for $i {
            fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
                (self.to_token_stream(), Default::default())
            }
        }
    };
}

macro_rules! braced_split_args {
    ($i:path$(,)?) => {
        impl SplitArgs for $i {
            fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
                (quote! { { #self } }, Default::default())
            }
        }
    };
}

default_split_args!(syn::ExprArray);
braced_split_args!(syn::ExprAssign);
default_split_args!(syn::ExprAsync);
default_split_args!(syn::ExprAwait);
braced_split_args!(syn::ExprBinary);
default_split_args!(syn::ExprBlock);
braced_split_args!(syn::ExprBreak);

impl SplitArgs for syn::ExprCall {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        let ExprCall {
            attrs, func, args, ..
        } = self;
        let attrs = attrs_to_tokens(attrs);
        (
            quote! {
                #attrs #func
            },
            expr_args_to_token_args(args),
        )
    }
}

// MOST OF THE SPLITS DON'T MEAN ANYTHING but are implemented anyways just to compile.

braced_split_args!(syn::ExprCast);
braced_split_args!(syn::ExprClosure);
braced_split_args!(syn::ExprConst);
braced_split_args!(syn::ExprContinue);
default_split_args!(syn::ExprField);
braced_split_args!(syn::ExprForLoop);
braced_split_args!(syn::ExprGroup);
default_split_args!(syn::ExprIf);
default_split_args!(syn::ExprIndex);
default_split_args!(syn::ExprInfer);
braced_split_args!(syn::ExprLet);
default_split_args!(syn::ExprLit);
braced_split_args!(syn::ExprLoop);
default_split_args!(syn::ExprMacro);
default_split_args!(syn::ExprMatch);

impl SplitArgs for syn::ExprMethodCall {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        let ExprMethodCall {
            attrs,
            method,
            receiver,
            turbofish,
            args,
            ..
        } = self;

        let attrs = attrs_to_tokens(&attrs);
        (
            quote! { #attrs #method . #receiver #turbofish},
            expr_args_to_token_args(args),
        )
    }
}

default_split_args!(syn::ExprParen);
default_split_args!(syn::ExprPath);
braced_split_args!(syn::ExprRange);
braced_split_args!(syn::ExprReference);
default_split_args!(syn::ExprRepeat);
braced_split_args!(syn::ExprReturn);
default_split_args!(syn::ExprStruct);
braced_split_args!(syn::ExprTry);
braced_split_args!(syn::ExprTryBlock);
default_split_args!(syn::ExprTuple);
braced_split_args!(syn::ExprUnary);
braced_split_args!(syn::ExprUnsafe);

// Expr::Verbatim
impl SplitArgs for TokenStream2 {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        (self.clone(), Default::default())
    }
}

braced_split_args!(syn::ExprWhile);
braced_split_args!(syn::ExprYield);

impl SplitArgs for Expr {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        use Expr::*;
        match self {
            Array(v) => v.split_args(),
            Assign(v) => v.split_args(),
            Async(v) => v.split_args(),
            Await(v) => v.split_args(),
            Binary(v) => v.split_args(),
            Block(v) => v.split_args(),
            Break(v) => v.split_args(),
            Call(v) => v.split_args(),
            Cast(v) => v.split_args(),
            Closure(v) => v.split_args(),
            Const(v) => v.split_args(),
            Continue(v) => v.split_args(),
            Field(v) => v.split_args(),
            ForLoop(v) => v.split_args(),
            Group(v) => v.split_args(),
            If(v) => v.split_args(),
            Index(v) => v.split_args(),
            Infer(v) => v.split_args(),
            Let(v) => v.split_args(),
            Lit(v) => v.split_args(),
            Loop(v) => v.split_args(),
            Macro(v) => v.split_args(),
            Match(v) => v.split_args(),
            MethodCall(v) => v.split_args(),
            Paren(v) => v.split_args(),
            Path(v) => v.split_args(),
            Range(v) => v.split_args(),
            Reference(v) => v.split_args(),
            Repeat(v) => v.split_args(),
            Return(v) => v.split_args(),
            Struct(v) => v.split_args(),
            Try(v) => v.split_args(),
            TryBlock(v) => v.split_args(),
            Tuple(v) => v.split_args(),
            Unary(v) => v.split_args(),
            Unsafe(v) => v.split_args(),
            Verbatim(v) => v.split_args(),
            While(v) => v.split_args(),
            Yield(v) => v.split_args(),
            &_ => todo!("unknown new syntax for syn::Expr"),
        }
    }
}

/// A [`syn::ExprMethodCall`] without the parens and `turbofish` (unable to parse with [`Expr`]).
#[derive(Clone)]
pub(crate) struct TurboMethod {
    pub attrs: Vec<Attribute>,
    pub receiver: Expr,
    pub method: Ident,
    pub turbofish: AngleBracketedGenericArguments,
}

impl Parse for TurboMethod {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let v = syn::parse_str::<ExprMethodCall>(&(input.to_string() + "()"))?;

        let Some(turbofish) = v.turbofish else {
            return Err(syn::Error::new(
                v.method.span(),
                "expected `::<...>` after method name",
            ));
        };

        Ok(Self {
            attrs: v.attrs,
            receiver: *v.receiver,
            method: v.method,
            turbofish,
        })
    }
}

impl ToTokens for TurboMethod {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for i in self.attrs.iter() {
            i.to_tokens(tokens);
        }
        self.receiver.to_tokens(tokens);
        tokens.extend(quote! { . });
        self.method.to_tokens(tokens);
        self.turbofish.to_tokens(tokens);
    }
}

impl SplitArgs for TurboMethod {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        let Self {
            attrs,
            receiver,
            method,
            turbofish,
        } = self;
        let attrs = attrs_to_tokens(attrs);
        (
            quote! { #attrs #receiver . #method #turbofish },
            Default::default(),
        )
    }
}

/// Extra expressions not defined in [`Expr`].
#[derive(Clone)]
pub(crate) enum ExtraExpr {
    TurboMethod(TurboMethod),
}

impl Parse for ExtraExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if let Ok(turbo) = input.parse() {
            Ok(Self::TurboMethod(turbo))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for ExtraExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ExtraExpr::TurboMethod(v) => v.to_tokens(tokens),
        }
    }
}

impl SplitArgs for ExtraExpr {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        match self {
            Self::TurboMethod(v) => v.split_args(),
        }
    }
}

/// The union of [`Expr`] and [`ExtraExpr`].
#[derive(Clone)]
pub(crate) enum ExtendedExpr {
    Expr(Expr),
    Extra(ExtraExpr),
}

impl Parse for ExtendedExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // note that the extra values are the first to be tried.
        Ok(if let Ok(extra) = input.parse() {
            Self::Extra(extra)
        } else {
            // TODO print value options and helpers in the console output if failure occurs
            Self::Expr(input.parse()?)
        })
    }
}

impl ToTokens for ExtendedExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Expr(v) => v.to_tokens(tokens),
            Self::Extra(v) => v.to_tokens(tokens),
        }
    }
}

impl SplitArgs for ExtendedExpr {
    fn split_args(&self) -> (TokenStream2, Punctuated<TokenStream2, syn::Token![,]>) {
        match self {
            Self::Extra(v) => v.split_args(),
            Self::Expr(v) => v.split_args(),
        }
    }
}
