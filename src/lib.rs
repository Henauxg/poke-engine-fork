// All four engines are compiled into a single library and the generation is chosen at
// runtime. Generations 1, 2 and 3 are separate implementations (they predate abilities,
// items and the physical/special split); generations 4-9 are served by the `genx` engine,
// which is generic over `const GEN: u8`.
pub mod gen1;
pub mod gen2;
pub mod gen3;
pub mod genx;

/// The shared crate-root code refers to engine *types* (the unified enums and
/// `MoveChoice`) as `crate::engine::...`. All four engines share genx's definitions of
/// those types, so this alias points at genx. Engine *functions* differ per generation
/// and are reached through [`gen_dispatch`] instead.
pub use crate::genx as engine;

pub mod choices;
pub mod gen_dispatch;
pub mod instruction;
pub mod io;
pub mod mcts;
pub mod mcts_threaded;
pub mod pokemon;
pub mod search;
pub mod state;

#[macro_export]
macro_rules! assert_unique_feature {
    () => {};
    ($first:tt $(,$rest:tt)*) => {
        $(
            #[cfg(all(feature = $first, feature = $rest))]
            compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
        )*
        assert_unique_feature!($($rest),*);
    }
}

#[macro_export]
macro_rules! define_enum_with_from_str {
    // Case when a default variant is provided
    (
        #[repr($repr:ident)]
        $(#[$meta:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        },
        default = $default_variant:ident
    ) => {
        #[repr($repr)]
        $(#[$meta])*
        pub enum $name {
            $($variant),+
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input.to_uppercase().as_str() {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )+
                    _ => Ok($name::$default_variant),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl From<$repr> for $name {
            fn from(value: $repr) -> $name {
                match value {
                    $(
                        x if x == $name::$variant as $repr => $name::$variant,
                    )+
                    _ => $name::$default_variant,
                }
            }
        }
        impl Into<$repr> for $name {
            fn into(self) -> $repr {
                self as $repr
            }
        }
    };

    // Case when no default variant is provided
    (
        #[repr($repr:ident)]
        $(#[$meta:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        }
    ) => {
        #[repr($repr)]
        $(#[$meta])*
        pub enum $name {
            $($variant),+
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input.to_uppercase().as_str() {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )+
                    _ => panic!("Invalid {}: {}", stringify!($name), input.to_uppercase().as_str()),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl From<$repr> for $name {
            fn from(value: $repr) -> $name {
                match value {
                    $(
                        x if x == $name::$variant as $repr => $name::$variant,
                    )+
                    _ => panic!("Invalid {}: {}", stringify!($name), value),
                }
            }
        }
        impl Into<$repr> for $name {
            fn into(self) -> $repr {
                self as $repr
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Generation support.
//
// One library serves generations 1..=9, selected at runtime. Generations 4-9 come from
// the `genx` engine, which is generic over `const GEN: u8`: the compiler monomorphizes
// one copy per generation the consumer instantiates, and every `GEN == N` / `GEN >= N`
// branch folds to a constant at that point, so there is no hot-path cost versus the old
// per-feature builds. Generations 1-3 remain separate engine implementations, reached
// through the same runtime facade (see `gen_dispatch`).
// ---------------------------------------------------------------------------

/// Lowest generation this library supports.
pub const MIN_GEN: u8 = 1;
/// Highest generation this library supports.
pub const MAX_GEN: u8 = 9;
/// Lowest generation served by the const-generic `genx` engine. Generations below this
/// are served by their own engine implementations (`gen1`, `gen2`, `gen3`).
pub const MIN_GENX_GEN: u8 = 4;
/// Default generation used by convenience entry points that take no generation
/// (e.g. the CLI when `--gen` is omitted). Newest supported generation.
pub const DEFAULT_GEN: u8 = 9;

/// Compile-fail guard for the `genx` engine's `const GEN: u8`. Referencing
/// `AssertGenInRange::<GEN>::CHECK` inside a generic genx function turns a `GEN` outside
/// `4..=9` into a post-monomorphization compile error instead of silently-wrong
/// behavior. Generations 1-3 are served by their own engines, not by genx.
pub struct AssertGenInRange<const GEN: u8>;
impl<const GEN: u8> AssertGenInRange<GEN> {
    pub const CHECK: () = assert!(
        GEN >= MIN_GENX_GEN && GEN <= MAX_GEN,
        "the genx engine only serves generations 4..=9"
    );
}
