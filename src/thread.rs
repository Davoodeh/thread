//! The main struct of the program.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Token,
};

use crate::{
    either::prelude::*,
    extended_syn::{ExtendedExpr, SplitArgs},
    misc::{CondType, LetAlias, Pattern, Placement},
};

/// The starting definitions of the a thread macro before the instruction set.
pub(crate) struct ThreadMacro {
    pattern: Option<Pattern>,
    given_initial_expr: Expr,
    alias_or_placement: Either<Ident, Placement>,
}

impl ThreadMacro {
    /// Add a new argument to the args list based on the alias or placement.
    fn add_arg(
        &self,
        args: &mut Punctuated<TokenStream2, Token![,]>,
        new_arg: &TokenStream2,
        skip_if_placement_and_true: bool,
    ) {
        match &self.alias_or_placement {
            Right(Placement::First) if !skip_if_placement_and_true => {
                args.insert(0, new_arg.clone())
            }
            Right(Placement::Last) if !skip_if_placement_and_true => args.push(new_arg.clone()),
            Left(alias) if args.is_empty() => args.push(alias.to_token_stream()),
            _ => {} // assumed alias is used correctly in args
        };
    }

    /// Resolve which assume alias inputs are valid (if alias used) and don't return output.
    ///
    /// In other words, if there are aliases used, assign values to aliases and assume it is
    /// defined. Else, return the value directly.
    fn resolve_set_alias(&self, mut instructions: TokenStream2, is_map: bool) -> TokenStream2 {
        let Either::Left(alias) = &self.alias_or_placement else {
            return instructions;
        };
        let given_initial_expr = &self.given_initial_expr;

        if !is_map {
            instructions.extend(quote! { ; #alias });
        }

        quote! {
            {
                let #alias = #given_initial_expr;
                #instructions
            }
        }
    }

    /// Add an alias to the results or not.
    fn resolve_instruction_alias(
        &self,
        given_argument: &TokenStream2,
        results: TokenStream2,
    ) -> TokenStream2 {
        let Left(alias) = &self.alias_or_placement else {
            return results;
        };

        let results = quote! { let #alias = #results; };

        // to prevent extra semicolons
        if given_argument.is_empty() {
            return results;
        }
        quote! { #given_argument; #results }
    }

    fn parse_map_instructions(&self, input: ParseStream) -> syn::Result<TokenStream2> {
        self.parse_instructions(
            input,
            ExtendedExpr::parse,
            |last_expr, expr| {
                let (func, mut args) = expr.split_args();
                let no_arg = last_expr.is_empty();

                let map_alias = quote! { i };
                self.add_arg(&mut args, &map_alias, no_arg);
                let final_alias = self
                    .alias_or_placement
                    .left()
                    .map(ToTokens::to_token_stream)
                    .unwrap_or(map_alias);

                // If there is an alias passed to the function in `alias_or_placement`, then
                // necessarily, `given_argument` is empty and must be replaced with that alias.
                let prefix = if no_arg { &final_alias } else { &last_expr };

                quote! { #prefix.map(|#final_alias| #func(#args)) }
            },
            true,
        )
    }

    fn parse_no_map_instructions(&self, input: ParseStream) -> syn::Result<TokenStream2> {
        self.parse_instructions(
            input,
            ExtendedExpr::parse,
            |last_expr, expr| {
                let (func, mut args) = expr.split_args();
                self.add_arg(&mut args, &last_expr, last_expr.is_empty());
                self.resolve_instruction_alias(&last_expr, quote! { #func(#args) })
            },
            false,
        )
    }

    fn parse_cond_instructions(&self, input: ParseStream) -> syn::Result<TokenStream2> {
        self.parse_instructions(
            input,
            |input| {
                let cond = input.parse::<Expr>()?;
                input.parse::<Token![=>]>()?;
                let expr = input.parse::<ExtendedExpr>()?;
                Ok((cond, expr))
            },
            |mut last_expr, (cond, expr)| {
                let (func, mut args) = expr.split_args();
                let no_arg = last_expr.is_empty();

                if !no_arg {
                    if let Some(Pattern::Cond(CondType::CondClone)) = &self.pattern {
                        last_expr = quote! { (#last_expr.clone()) };
                    }
                }

                self.add_arg(&mut args, &last_expr, no_arg);

                self.resolve_instruction_alias(
                    &last_expr,
                    quote! {
                        {
                            if #cond {
                                #func(#args)
                            } else {
                                #last_expr
                            }
                        }
                    },
                )
            },
            false,
        )
    }

    /// Parse a list of instructions. Single source of truth for instruction parsers.
    fn parse_instructions<T, F>(
        &self,
        input: ParseStream,
        parser: fn(ParseStream) -> syn::Result<T>,
        tokenizer: F,
        is_map: bool,
    ) -> syn::Result<TokenStream2>
    where
        F: FnMut(TokenStream2, T) -> TokenStream2,
    {
        let instructions = Punctuated::<T, Token![,]>::parse_terminated_with(input, parser)?;

        if instructions.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "expected some functions as pipe",
            ));
        }

        let expr = instructions.into_iter().fold(
            // The very first input to the token functions (the actual initial_expr or nothing if alias).
            match self.alias_or_placement {
                Left(_) => Default::default(),
                Right(_) => self.given_initial_expr.to_token_stream(),
            },
            tokenizer,
        );

        Ok(self.resolve_set_alias(expr, is_map))
    }

    /// Parse tokens and generate the valid output.
    pub fn generate_tokens(input: ParseStream) -> syn::Result<TokenStream> {
        let results = Self::parse_preamble(input)?;

        input.parse::<Token![in]>()?;

        Ok(match &results.pattern {
            Some(Pattern::Cond(_)) => results.parse_cond_instructions(input)?,
            None => results.parse_no_map_instructions(input)?,
            Some(Pattern::Map(_)) => results.parse_map_instructions(input)?,
        }
        .into())
    }

    /// Create an instance by parsing up to `in`.
    pub fn parse_preamble(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![let]) {
            input.parse::<LetAlias>()?.into()
        } else {
            let pattern = input.parse().ok();
            let initial_expr = input.parse()?;
            let placement = input.parse().unwrap_or_default();

            Self {
                pattern,
                given_initial_expr: initial_expr,
                alias_or_placement: Right(placement),
            }
        })
    }
}

impl From<LetAlias> for ThreadMacro {
    fn from(value: LetAlias) -> Self {
        Self {
            pattern: value.pattern,
            given_initial_expr: value.value,
            alias_or_placement: Left(value.alias),
        }
    }
}
