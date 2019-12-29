//! # Cuneiform: Cache optimizations for Rust, revived from the slabs of Sumer.
//! [![Cuneiform](https://github.com/vertexclique/cuneiform/raw/master/img/cuneiform-logo.png)](https://github.com/vertexclique/cuneiform)
//!
//! This crate provides a proc macro attribute to optimize CPU cache operations for user defined structs.
//! Cuneiform can take various arguments at attribute position:
//! * `hermetic = true|false` (default is `true` when `#[cuneiform]`)
//! * Hermetic enables cuneiform to detect cache sizes from OSes which have API to fetch.
//! * Currently, hermetic argument works only Linux kernel 2.6.32 and above.
//! * If system is different than supported systems it falls back to `slab`s.
//! * `slab = "board_or_architecture_name` (e.g. `#[cuneiform(slab = "powerpc_mpc8xx")]`)
//!     * Slabs are either embedded system boards or other specific architecture.
//!     * Slab checking have two stages:
//!         * First, it checks the given board/architecture if exist.
//!         * If not slabs fall back to Rust supported architectures.
//!         * Still architecture is not detected, it will fall back to default values.
//! * `force = u8` (e.g. `#[cuneiform(force = 16)]`)
//!     * Forces a given cache alignment. Overrides all other systems mentioned above.
//!
//! ```toml
//! [dependencies]
//! cuneiform = "0.1"
//! ```
//!
//! ## Examples
//! Basic usage can be:
//! ```rust
//! use cuneiform::*;
//!
//! // Defaults to `hermetic = true`
//! #[cuneiform]
//! pub struct Varying {
//!     data: u8,
//!     data_2: u16,
//! }
//! ```
//!
//! Targeting specific architecture:
//! ```rust
//! use cuneiform::*;
//!
//! #[cuneiform(slab = "krait")]
//! pub struct SlabBased {
//!     data: u8,
//!     data_2: u16,
//! }
//! ```
//!
//! Overriding the default cache alignment:
//! ```rust
//! use cuneiform::*;
//!
//! #[cuneiform(force = 16)]
//! pub struct Forced {
//!     data: u8,
//!     data_2: u16,
//! }
//! ```

#![doc(
    html_logo_url = "https://github.com/vertexclique/cuneiform/raw/master/img/cuneiform-logo.png"
)]
// Force missing implementations
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use heapless::consts::*;
use heapless::String;
use quote::*;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, DeriveInput, Index, LitBool, LitInt, LitStr, Token};

mod detection;
mod slabs;

///
/// Inference arguments
pub(crate) struct CuneiformArgs {
    /// Inherit coherency size targeting from the current machine that the compiler runs. (Default)
    pub(crate) hermetic: bool,
    /// Force the slab that is going to be used.
    pub(crate) slab: String<U64>,
    /// Force size to a specific amount. Overrides any other parameter.
    pub(crate) force: isize,
}

impl CuneiformArgs {
    fn new() -> Self {
        CuneiformArgs {
            hermetic: true,
            slab: String::from(""),
            force: !0,
        }
    }

    fn with_hermetic(&mut self, hermetic: bool) {
        self.hermetic = hermetic;
    }

    fn with_slab(&mut self, slab: String<U64>) {
        self.slab = slab;
    }

    fn with_force(&mut self, force: isize) {
        self.force = force;
    }
}

mod cunei_keywords {
    syn::custom_keyword!(hermetic);
    syn::custom_keyword!(slab);
    syn::custom_keyword!(force);
}

///
/// Syn parse for Cuneiform
impl Parse for CuneiformArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut cargs = CuneiformArgs::new();

        if input.parse::<cunei_keywords::hermetic>().is_ok() {
            input.parse::<Token![=]>()?;
            let hermetic: LitBool = input.parse()?;
            cargs.with_hermetic(hermetic.value);
        }

        if input.parse::<cunei_keywords::slab>().is_ok() {
            input.parse::<Token![=]>()?;
            let slab: LitStr = input.parse()?;
            cargs.with_slab(String::from(slab.value().as_str()));
        }

        if input.parse::<cunei_keywords::force>().is_ok() {
            input.parse::<Token![=]>()?;
            let force_lit: LitInt = input.parse()?;
            let force = force_lit.base10_parse::<isize>()?;
            cargs.with_force(force);
        }

        Ok(cargs)
    }
}

///
/// Entry point for cuneiform proc macro attribute.
///
/// Cuneiform allows structs to be optimized for the specific cache line sizes.
///
/// # Example
///
/// ```rust
/// use cuneiform::*;
///
/// #[cuneiform]
/// pub struct Varying {
///     data: u8,
///     data_2: u16,
/// }
/// ```
#[proc_macro_attribute]
pub fn cuneiform(args: TokenStream, input: TokenStream) -> TokenStream {
    let pargs = parse_macro_input!(args as CuneiformArgs);
    let input = parse_macro_input!(input as DeriveInput);

    let frep = crate::slabs::fetch(pargs);
    let frep = Index::from(frep as usize);
    TokenStream::from(quote! {
        // Preserve the input struct unchanged in the output.
        #[repr(align(#frep))]
        #input
    })
}
