//! This crate includes [Clojure's threading macros](https://clojure.org/guides/threading_macros).

// NOTE doto is not written here since Rust syntax makes it obsolete

// TODO add the mut and ref modifier to macro

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod either;
mod extended_syn;
mod misc;
mod thread;

use thread::ThreadMacro;

/// The only macro to be used with this crate includes all the other macros.
///
/// These macros include:
/// - The thread-first macro (`->`)
/// - thread-last (`->>`)
/// - and thread-as (`as->`).
///
/// Also, `some`, `cond` and `ok` (Rust only) are added in the three variants above (`*as` for the
/// latter three is only in Rust).
#[proc_macro]
pub fn thread(tokens: TokenStream) -> TokenStream {
    parse_macro_input!(tokens with ThreadMacro::generate_tokens)
}
