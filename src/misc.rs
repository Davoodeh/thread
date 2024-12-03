//! Holds anything else.
use crate::extended_syn::{
    parse_parens,
    token::{KwCond, KwCondClone, KwFirst, KwLast, KwOk, KwSome},
};

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Token,
};

macro_rules! keyword_enum {
    (
        $(#[$attr:meta])*
        $name:ident {
            $(#[$var_attr:meta])*
            $var:ident
            $(,
              $(#[$vars_attr:meta])*
              $vars:ident)*$(,)?
        }
    ) => {
        $(#[$attr])*
        pub(crate) enum $name {
            $(#[$var_attr])*
            $var
            $(,
              $(#[$vars_attr])*
              $vars)*
        }

        impl Parse for $name {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let lookahead = input.lookahead1();
                keyword_enum!(_cond, lookahead, input, $var$(, $vars)*);
                return Err(lookahead.error());
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream2) {
                match self {
                    Self::$var => <keyword_enum!(_kw $var)>::default().to_tokens(tokens),
                    $(Self::$vars => <keyword_enum!(_kw $vars)>::default().to_tokens(tokens))*
                }
            }
        }
    };

    (_kw $var:ident) => {
        paste::paste! { [<Kw $var>] }
    };

    (_cond, $lookahead:ident, $input:ident, $var:ident$(, $vars:ident)*) => {
        keyword_enum!(__cond, $lookahead, $input, $var);
        keyword_enum!(_cond, $lookahead, $input $(, $vars)*);
    };

    (_cond, $lookahead:ident, $input:ident) => {}; // end loop condition

    (__cond, $lookahead:ident, $input:ident, $var:ident) => {
        if $lookahead.peek(keyword_enum!(_kw $var)) {
            $input.parse::<keyword_enum!(_kw $var)>().unwrap();
            return Ok(Self::$var);
        }
    };
}

keyword_enum! {
    /// Keywords that are processed with map functions with functional utilities.
    #[derive(Debug)]
    Map {
        Some,
        Ok,
    }
}

keyword_enum! {
    /// Cond pattern rather than a magic.
    #[derive(Debug)]
    CondType {
        /// Values to be used as provided to the magic function.
        Cond,
        /// Values that need to be cloned on each use.
        CondClone,
    }
}

keyword_enum! {
    /// Where an argument must be inserted in a list.
    Placement {
        First,
        Last,
    }
}

impl Default for Placement {
    fn default() -> Self {
        Self::First
    }
}

/// Values before a let or match or at the start of a phrase to signify a change in process.
pub(crate) enum Pattern {
    Map(Map),
    Cond(CondType),
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse()
            .map(Pattern::Cond)
            .or_else(|_| input.parse().map(Pattern::Map))
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Map(v) => v.to_tokens(tokens),
            Self::Cond(v) => v.to_tokens(tokens),
        }
    }
}

/// A limited [`LetExpr`] tailored for this crate.
pub(crate) struct LetAlias {
    /// Keywords for pattern matching behind a let keyword: `let PAT(i) = value`.
    pub pattern: Option<Pattern>,
    pub alias: Ident,
    pub value: Expr,
}

impl Parse for LetAlias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![let]>()?;
        let pattern = input.parse().ok();

        let alias = parse_parens(input, pattern.is_some())?;

        input.parse::<Token![=]>()?;

        let value = input.parse()?;

        Ok(Self {
            pattern,
            alias,
            value,
        })
    }
}
