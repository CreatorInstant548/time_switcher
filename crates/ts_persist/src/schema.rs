//! Flat schema: no recursive type, so the decoding depth is fixed at compile time (Q2).

use core::fmt;
use core::marker::PhantomData;

use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// spec: carnet architecture-securite.md SEC0-23, §9.13
/// Nesting depth of a schema type: `0` for a leaf, one more than its deepest field for a
/// structure, one more than its element for a [`Seq`].
///
/// Every structure of the format is declared through `flat_schema!`, which computes the
/// depth from the field types. A recursive type would make its own depth depend on itself,
/// which does not compile; a type with no `Schema` impl (`Vec`, `Box`, `Option`…) cannot be a
/// field. The depth is therefore bounded by construction, and checked against
/// [`MAX_SCHEMA_DEPTH`] at compile time.
pub trait Schema {
    /// Nesting depth.
    const DEPTH: usize;
}

/// Deepest nesting allowed in the format.
pub const MAX_SCHEMA_DEPTH: usize = 4;

macro_rules! leaf {
    ($($t:ty),*) => {$(
        impl Schema for $t {
            const DEPTH: usize = 0;
        }
    )*};
}
leaf!(u8, u16, u32, u64, i64, String, [u8; 32]);

/// Largest value of `xs`, `0` if empty.
pub(crate) const fn max_depth(xs: &[usize]) -> usize {
    match xs.split_first() {
        Some((&x, rest)) => {
            let r = max_depth(rest);
            if x > r { x } else { r }
        }
        None => 0,
    }
}

/// Declares a structure of the format: the structure, its [`Schema`] depth, and the
/// compile-time check against [`MAX_SCHEMA_DEPTH`].
macro_rules! flat_schema {
    (
        $(#[$m:meta])*
        pub struct $name:ident { $($(#[$fm:meta])* pub $field:ident : $ty:ty),* $(,)? }
    ) => {
        $(#[$m])*
        pub struct $name { $($(#[$fm])* pub $field: $ty),* }

        impl $crate::schema::Schema for $name {
            const DEPTH: usize =
                1 + $crate::schema::max_depth(&[$(<$ty as $crate::schema::Schema>::DEPTH),*]);
        }

        const _: () = assert!(
            <$name as $crate::schema::Schema>::DEPTH <= $crate::schema::MAX_SCHEMA_DEPTH,
            "schema too deep"
        );
    };
}
pub(crate) use flat_schema;

// spec: carnet architecture-securite.md §9.13
/// Sequence of the format. Unlike `Vec`, its decoding never allocates from the length the
/// input announces (Q2): the vector grows only with the elements actually decoded, from a
/// buffer that is already bounded. The caller then checks the count against its
/// [`crate::Limits`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seq<T>(pub Vec<T>);

impl<T> Default for Seq<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T: Schema> Schema for Seq<T> {
    const DEPTH: usize = T::DEPTH + 1;
}

impl<T: Serialize> Serialize for Seq<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_seq(&self.0)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Seq<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Elements<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Elements<T> {
            type Value = Seq<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Seq<T>, A::Error> {
                let mut out = Vec::new();
                while let Some(x) = seq.next_element()? {
                    out.push(x);
                }
                Ok(Seq(out))
            }
        }

        d.deserialize_seq(Elements(PhantomData))
    }
}
