#[cfg(feature = "gen1")]
#[path = "gen1/mod.rs"]
pub mod engine;

#[cfg(feature = "gen2")]
#[path = "gen2/mod.rs"]
pub mod engine;

#[cfg(feature = "gen3")]
#[path = "gen3/mod.rs"]
pub mod engine;

// All other generations
#[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
#[path = "genx/mod.rs"]
pub mod engine;

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

// gen1/gen2/gen3 are separate compile-time engines and stay mutually exclusive.
assert_unique_feature!("gen1", "gen2", "gen3");

// `champions`/`bss` are genx-path features (const-generic engine); they cannot combine
// with the standalone gen1/gen2/gen3 engines.
#[cfg(all(
    feature = "champions",
    any(feature = "gen1", feature = "gen2", feature = "gen3")
))]
compile_error!(
    "`champions`/`bss` require the genx path and cannot be combined with gen1/gen2/gen3"
);

// ---------------------------------------------------------------------------
// Const-generic generation support for the `genx` engine.
//
// `genx` serves generations 4..=9 from a single compiled library. Each entry point is
// generic over `const GEN: u8`; the compiler monomorphizes one copy per generation the
// consumer actually instantiates, and every `GEN == N` / `GEN >= N` branch folds to a
// constant at that point, so there is no hot-path cost versus the old per-feature build.
// ---------------------------------------------------------------------------

/// Lowest generation the const-generic `genx` engine supports.
pub const MIN_GEN: u8 = 4;
/// Highest generation the const-generic `genx` engine supports.
pub const MAX_GEN: u8 = 9;
/// Default generation used by convenience entry points that take no generation
/// (e.g. the CLI when `--gen` is omitted). Newest supported generation.
pub const DEFAULT_GEN: u8 = 9;

/// Compile-fail guard for `const GEN: u8`. Referencing `AssertGenInRange::<GEN>::CHECK`
/// inside a generic function turns a `GEN` outside `4..=9` into a
/// post-monomorphization compile error instead of silently-wrong behavior.
pub struct AssertGenInRange<const GEN: u8>;
impl<const GEN: u8> AssertGenInRange<GEN> {
    pub const CHECK: () = assert!(
        GEN >= MIN_GEN && GEN <= MAX_GEN,
        "genx only supports generations 4..=9"
    );
}
