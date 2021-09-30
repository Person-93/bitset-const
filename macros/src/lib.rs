use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitInt, Token, Visibility};

#[proc_macro]
pub fn bitset(item: TokenStream) -> TokenStream {
    let BitSet { vis, name, bits } = parse_macro_input!(item as BitSet);
    let bits = match bits.base10_parse::<usize>() {
        Ok(bits) => bits,
        Err(err) => return err.into_compile_error().into(),
    };
    const BYTE_SIZE: usize = u128::BITS as usize;
    let mut bytes = bits / BYTE_SIZE;
    if bits % BYTE_SIZE > 0 {
        bytes += 1;
    }

    let expanded = quote! {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        #vis struct #name([u128; #bytes]);

        impl #name {
            #[inline]
            pub const fn new() -> Self {
                Self([0; #bytes])
            }

            #[inline]
            pub const fn get(&self, i: usize) -> bool {
                let (i, mask) = Self::__calc_offset(i);
                self.0[i] & mask != 0
            }

            #[inline]
            pub fn set(&mut self, i: usize, value: bool) {
                let (i, mask) = Self::__calc_offset(i);
                self.0[i] |= mask;
            }

            #[inline]
            pub fn count(&self) -> u64 {
                self.0.iter().fold(0u64, |mut acc, curr| {
                    acc += curr.count_ones() as u64;
                    acc
                })
            }

            #[doc(hidden)]
            #[inline]
            const fn __calc_offset(i: usize) -> (usize, u128) {
                (i / #BYTE_SIZE, 1u128 << (i % #BYTE_SIZE))
            }
        }

        impl core::fmt::Binary for #name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_> ) -> core::fmt::Result {
                for byte in self.0 {
                    core::fmt::Binary::fmt(&byte, f)?;
                }
                Ok(())
            }
        }

        impl core::ops::BitAnd for #name {
            type Output = Self;
            #[inline]
            fn bitand(mut self, rhs: Self) -> Self::Output {
                self &= rhs;
                self
            }
        }

        impl core::ops::BitAndAssign for #name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                for (this, other) in self.0.iter_mut().zip(rhs.0.iter()) {
                    *this &= other;
                }
            }
        }

        impl core::ops::BitOr for #name {
            type Output = Self;
            #[inline]
            fn bitor(mut self, rhs: Self) -> Self::Output {
                self |= rhs;
                self
            }
        }

        impl core::ops::BitOrAssign for #name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                for (this, other) in self.0.iter_mut().zip(rhs.0.iter()) {
                    *this |= other;
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Parse)]
struct BitSet {
    vis: Visibility,
    #[prefix(Token![struct])]
    name: Ident,
    #[prefix(Token![:])]
    bits: LitInt,
}
