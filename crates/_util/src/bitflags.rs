#[macro_export]
macro_rules! bitflags {
    (
        #[derive($($derive:meta),*)]
        pub struct $name:ident($vis:vis $ty:ty) {
            $($(#[doc = $doc:expr])* const $flag:ident = $value:expr;)*
        }
    ) => {
        #[derive($($derive),*)]
        #[repr(transparent)]
        pub struct $name($vis $ty);

        impl $name {
            pub const NONE: Self = Self(0);
            $($(#[doc = $doc])* pub const $flag: Self = Self($value);)*

            #[inline]
            pub fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0 && other.0 != 0
            }

            #[inline]
            pub fn intersects(&self, other: Self) -> bool {
                (self.0 & other.0) != 0
            }

            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.0 |= other.0;
            }

            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.0 &= !other.0;
            }

            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0 ^= other.0;
            }
        }

        // Standard Bitwise Operators
        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
        }
        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
        }
        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
        }
        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self { Self(!self.0) }
        }
        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
        }
        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) { self.0 &= !rhs.0; }
        }
    };
}
