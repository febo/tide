#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use solana_program_error::ProgramError;
#[cfg(feature = "wincode")]
use wincode::{SchemaRead, SchemaWrite};
/// A public key (32 bytes).
pub type Pubkey = [u8; 32];
/// Account data structure for a token account.
///
/// This is a simplified version of the SPL Token account structure
/// to demonstrate the concept.
#[repr(C)]
#[wincode(assert_zero_copy)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// The delegate for this account.
    pub delegate: Pubkey,
    /// The account's state.
    pub state: u8,
    /// Padding bytes.
    _padding: [u8; 7],
    /// Native token amount.
    pub native_amount: u64,
    /// The amount delegated.
    pub delegated_amount: u64,
    /// The close authority.
    pub close_authority: Pubkey,
}
#[automatically_derived]
impl ::core::fmt::Debug for Account {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "mint",
            "owner",
            "amount",
            "delegate",
            "state",
            "_padding",
            "native_amount",
            "delegated_amount",
            "close_authority",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.mint,
            &self.owner,
            &self.amount,
            &self.delegate,
            &self.state,
            &self._padding,
            &self.native_amount,
            &self.delegated_amount,
            &&self.close_authority,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Account", names, values)
    }
}
#[automatically_derived]
impl ::core::default::Default for Account {
    #[inline]
    fn default() -> Account {
        Account {
            mint: ::core::default::Default::default(),
            owner: ::core::default::Default::default(),
            amount: ::core::default::Default::default(),
            delegate: ::core::default::Default::default(),
            state: ::core::default::Default::default(),
            _padding: ::core::default::Default::default(),
            native_amount: ::core::default::Default::default(),
            delegated_amount: ::core::default::Default::default(),
            close_authority: ::core::default::Default::default(),
        }
    }
}
const _: () = {
    use ::wincode::{SchemaWrite, WriteResult, io::Writer, TypeMeta, config::Config};
    impl<WincodeConfig: Config> ::wincode::SchemaWrite<WincodeConfig> for Account {
        type Src = Self;
        #[allow(clippy::arithmetic_side_effects)]
        const TYPE_META: TypeMeta = if let (
            TypeMeta::Static { size: a, zero_copy: zc_a },
            TypeMeta::Static { size: b, zero_copy: zc_b },
            TypeMeta::Static { size: c, zero_copy: zc_c },
            TypeMeta::Static { size: d, zero_copy: zc_d },
            TypeMeta::Static { size: e, zero_copy: zc_e },
            TypeMeta::Static { size: f, zero_copy: zc_f },
            TypeMeta::Static { size: g, zero_copy: zc_g },
            TypeMeta::Static { size: h, zero_copy: zc_h },
            TypeMeta::Static { size: i, zero_copy: zc_i },
        ) = (
            <Pubkey as SchemaWrite<WincodeConfig>>::TYPE_META,
            <Pubkey as SchemaWrite<WincodeConfig>>::TYPE_META,
            <u64 as SchemaWrite<WincodeConfig>>::TYPE_META,
            <Pubkey as SchemaWrite<WincodeConfig>>::TYPE_META,
            <u8 as SchemaWrite<WincodeConfig>>::TYPE_META,
            <[u8; 7] as SchemaWrite<WincodeConfig>>::TYPE_META,
            <u64 as SchemaWrite<WincodeConfig>>::TYPE_META,
            <u64 as SchemaWrite<WincodeConfig>>::TYPE_META,
            <Pubkey as SchemaWrite<WincodeConfig>>::TYPE_META,
        ) {
            let serialized_size = a + b + c + d + e + f + g + h + i;
            let no_padding = serialized_size == core::mem::size_of::<Self>();
            TypeMeta::Static {
                size: serialized_size,
                zero_copy: no_padding && true && zc_a && zc_b && zc_c && zc_d && zc_e
                    && zc_f && zc_g && zc_h && zc_i,
            }
        } else {
            TypeMeta::Dynamic
        };
        #[inline]
        fn size_of(src: &Self::Src) -> WriteResult<usize> {
            if let TypeMeta::Static { size, .. } = <Self as SchemaWrite<
                WincodeConfig,
            >>::TYPE_META {
                return Ok(size);
            }
            let mut total = 0usize;
            total += <Pubkey as SchemaWrite<WincodeConfig>>::size_of(&src.mint)?;
            total += <Pubkey as SchemaWrite<WincodeConfig>>::size_of(&src.owner)?;
            total += <u64 as SchemaWrite<WincodeConfig>>::size_of(&src.amount)?;
            total += <Pubkey as SchemaWrite<WincodeConfig>>::size_of(&src.delegate)?;
            total += <u8 as SchemaWrite<WincodeConfig>>::size_of(&src.state)?;
            total += <[u8; 7] as SchemaWrite<WincodeConfig>>::size_of(&src._padding)?;
            total += <u64 as SchemaWrite<WincodeConfig>>::size_of(&src.native_amount)?;
            total
                += <u64 as SchemaWrite<WincodeConfig>>::size_of(&src.delegated_amount)?;
            total
                += <Pubkey as SchemaWrite<
                    WincodeConfig,
                >>::size_of(&src.close_authority)?;
            Ok(total)
        }
        #[inline]
        fn write(writer: &mut impl Writer, src: &Self::Src) -> WriteResult<()> {
            match <Self as SchemaWrite<WincodeConfig>>::TYPE_META {
                TypeMeta::Static { size, .. } => {
                    let writer = &mut unsafe { writer.as_trusted_for(size) }?;
                    <Pubkey as SchemaWrite<WincodeConfig>>::write(writer, &src.mint)?;
                    <Pubkey as SchemaWrite<WincodeConfig>>::write(writer, &src.owner)?;
                    <u64 as SchemaWrite<WincodeConfig>>::write(writer, &src.amount)?;
                    <Pubkey as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.delegate)?;
                    <u8 as SchemaWrite<WincodeConfig>>::write(writer, &src.state)?;
                    <[u8; 7] as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src._padding)?;
                    <u64 as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.native_amount)?;
                    <u64 as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.delegated_amount)?;
                    <Pubkey as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.close_authority)?;
                    writer.finish()?;
                }
                TypeMeta::Dynamic => {
                    <Pubkey as SchemaWrite<WincodeConfig>>::write(writer, &src.mint)?;
                    <Pubkey as SchemaWrite<WincodeConfig>>::write(writer, &src.owner)?;
                    <u64 as SchemaWrite<WincodeConfig>>::write(writer, &src.amount)?;
                    <Pubkey as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.delegate)?;
                    <u8 as SchemaWrite<WincodeConfig>>::write(writer, &src.state)?;
                    <[u8; 7] as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src._padding)?;
                    <u64 as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.native_amount)?;
                    <u64 as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.delegated_amount)?;
                    <Pubkey as SchemaWrite<
                        WincodeConfig,
                    >>::write(writer, &src.close_authority)?;
                }
            }
            Ok(())
        }
    }
};
const _: () = {
    use ::wincode::{config::ZeroCopy, SchemaRead, TypeMeta};
    const _assert_schema_read_impl: fn() = || {
        fn assert_schema_read_impl<
            'de,
            T: SchemaRead<'de, ::wincode::config::DefaultConfig>,
        >() {}
        assert_schema_read_impl::<Account>()
    };
    const _assert_field_zerocopy_impl: fn() = || {
        fn assert_field_zerocopy_impl<T: ZeroCopy<::wincode::config::DefaultConfig>>() {}
        assert_field_zerocopy_impl::<Pubkey>();
        assert_field_zerocopy_impl::<Pubkey>();
        assert_field_zerocopy_impl::<u64>();
        assert_field_zerocopy_impl::<Pubkey>();
        assert_field_zerocopy_impl::<u8>();
        assert_field_zerocopy_impl::<[u8; 7]>();
        assert_field_zerocopy_impl::<u64>();
        assert_field_zerocopy_impl::<u64>();
        assert_field_zerocopy_impl::<Pubkey>()
    };
    const _assert_no_padding: () = {
        if let TypeMeta::Static { size, .. } = <Account as SchemaRead<
            '_,
            ::wincode::config::DefaultConfig,
        >>::TYPE_META {
            if size != core::mem::size_of::<Account>() {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "wincode(assert_zero_copy) was applied to a type with padding",
                        ),
                    );
                };
            }
        } else {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "wincode(assert_zero_copy) was applied to a type with `TypeMeta::Dynamic`",
                    ),
                );
            };
        }
    };
};
const _: () = {
    use core::{ptr, mem::{self, MaybeUninit}};
    use ::wincode::{
        SchemaRead, ReadResult, TypeMeta, io::Reader, error,
        config::{Config, DefaultConfig, ZeroCopy},
    };
    struct Assert<const B: bool>;
    trait IsTrue {}
    impl IsTrue for Assert<true> {}
    unsafe impl<WincodeConfig: Config> ZeroCopy<WincodeConfig> for Account
    where
        for<'_wincode_internal> Pubkey: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> Pubkey: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> u64: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> Pubkey: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> u8: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> [u8; 7]: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> u64: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> u64: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> Pubkey: ZeroCopy<WincodeConfig>,
        for<'_wincode_internal> Assert<
            {
                core::mem::size_of::<Pubkey>() + core::mem::size_of::<Pubkey>()
                    + core::mem::size_of::<u64>() + core::mem::size_of::<Pubkey>()
                    + core::mem::size_of::<u8>() + core::mem::size_of::<[u8; 7]>()
                    + core::mem::size_of::<u64>() + core::mem::size_of::<u64>()
                    + core::mem::size_of::<Pubkey>() == core::mem::size_of::<Account>()
            },
        >: IsTrue,
    {}
    impl<'de, WincodeConfig: Config> SchemaRead<'de, WincodeConfig> for Account {
        type Dst = Self;
        #[allow(clippy::arithmetic_side_effects)]
        const TYPE_META: TypeMeta = if let (
            TypeMeta::Static { size: a, zero_copy: zc_a },
            TypeMeta::Static { size: b, zero_copy: zc_b },
            TypeMeta::Static { size: c, zero_copy: zc_c },
            TypeMeta::Static { size: d, zero_copy: zc_d },
            TypeMeta::Static { size: e, zero_copy: zc_e },
            TypeMeta::Static { size: f, zero_copy: zc_f },
            TypeMeta::Static { size: g, zero_copy: zc_g },
            TypeMeta::Static { size: h, zero_copy: zc_h },
            TypeMeta::Static { size: i, zero_copy: zc_i },
        ) = (
            <Pubkey as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <Pubkey as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <u64 as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <Pubkey as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <u8 as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <[u8; 7] as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <u64 as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <u64 as SchemaRead<'de, WincodeConfig>>::TYPE_META,
            <Pubkey as SchemaRead<'de, WincodeConfig>>::TYPE_META,
        ) {
            let serialized_size = a + b + c + d + e + f + g + h + i;
            let no_padding = serialized_size == core::mem::size_of::<Self>();
            TypeMeta::Static {
                size: serialized_size,
                zero_copy: no_padding && true && zc_a && zc_b && zc_c && zc_d && zc_e
                    && zc_f && zc_g && zc_h && zc_i,
            }
        } else {
            TypeMeta::Dynamic
        };
        #[inline]
        fn read(
            reader: &mut impl Reader<'de>,
            dst: &mut MaybeUninit<Self::Dst>,
        ) -> ReadResult<()> {
            struct DropGuard {
                init_count: u8,
                dst_ptr: *mut Account,
            }
            impl Drop for DropGuard {
                #[cold]
                fn drop(&mut self) {
                    let dst_ptr = self.dst_ptr;
                    let init_count = self.init_count;
                    match init_count {
                        0 => {}
                        1u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        2u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        3u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        4u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).delegate);
                                ptr::drop_in_place(&raw mut (*dst_ptr).amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        5u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).state);
                                ptr::drop_in_place(&raw mut (*dst_ptr).delegate);
                                ptr::drop_in_place(&raw mut (*dst_ptr).amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        6u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr)._padding);
                                ptr::drop_in_place(&raw mut (*dst_ptr).state);
                                ptr::drop_in_place(&raw mut (*dst_ptr).delegate);
                                ptr::drop_in_place(&raw mut (*dst_ptr).amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        7u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).native_amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr)._padding);
                                ptr::drop_in_place(&raw mut (*dst_ptr).state);
                                ptr::drop_in_place(&raw mut (*dst_ptr).delegate);
                                ptr::drop_in_place(&raw mut (*dst_ptr).amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        8u8 => {
                            unsafe {
                                ptr::drop_in_place(&raw mut (*dst_ptr).delegated_amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).native_amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr)._padding);
                                ptr::drop_in_place(&raw mut (*dst_ptr).state);
                                ptr::drop_in_place(&raw mut (*dst_ptr).delegate);
                                ptr::drop_in_place(&raw mut (*dst_ptr).amount);
                                ptr::drop_in_place(&raw mut (*dst_ptr).owner);
                                ptr::drop_in_place(&raw mut (*dst_ptr).mint);
                            }
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!("init_count out of bounds"),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match <Self as SchemaRead<'de, WincodeConfig>>::TYPE_META {
                TypeMeta::Static { size, .. } => {
                    let reader = &mut unsafe { reader.as_trusted_for(size) }?;
                    let dst_ptr = dst.as_mut_ptr();
                    let mut guard = DropGuard {
                        init_count: 0,
                        dst_ptr,
                    };
                    let init_count = &mut guard.init_count;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).mint).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).owner).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u64 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).amount).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).delegate).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u8 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).state).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <[u8; 7] as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr)._padding).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u64 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).native_amount)
                                .cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u64 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).delegated_amount)
                                .cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).close_authority)
                                .cast::<MaybeUninit<_>>()
                        },
                    )?;
                    mem::forget(guard);
                }
                TypeMeta::Dynamic => {
                    let dst_ptr = dst.as_mut_ptr();
                    let mut guard = DropGuard {
                        init_count: 0,
                        dst_ptr,
                    };
                    let init_count = &mut guard.init_count;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).mint).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).owner).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u64 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).amount).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).delegate).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u8 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).state).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <[u8; 7] as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr)._padding).cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u64 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).native_amount)
                                .cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <u64 as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).delegated_amount)
                                .cast::<MaybeUninit<_>>()
                        },
                    )?;
                    *init_count += 1;
                    <Pubkey as SchemaRead<
                        'de,
                        WincodeConfig,
                    >>::read(
                        reader,
                        unsafe {
                            &mut *(&raw mut (*dst_ptr).close_authority)
                                .cast::<MaybeUninit<_>>()
                        },
                    )?;
                    mem::forget(guard);
                }
            }
            Ok(())
        }
    }
};
const _: () = {
    use ::wincode::{config::ZeroCopy, SchemaRead, TypeMeta};
    const _assert_schema_read_impl: fn() = || {
        fn assert_schema_read_impl<
            'de,
            T: SchemaRead<'de, ::wincode::config::DefaultConfig>,
        >() {}
        assert_schema_read_impl::<Account>()
    };
    const _assert_field_zerocopy_impl: fn() = || {
        fn assert_field_zerocopy_impl<T: ZeroCopy<::wincode::config::DefaultConfig>>() {}
        assert_field_zerocopy_impl::<Pubkey>();
        assert_field_zerocopy_impl::<Pubkey>();
        assert_field_zerocopy_impl::<u64>();
        assert_field_zerocopy_impl::<Pubkey>();
        assert_field_zerocopy_impl::<u8>();
        assert_field_zerocopy_impl::<[u8; 7]>();
        assert_field_zerocopy_impl::<u64>();
        assert_field_zerocopy_impl::<u64>();
        assert_field_zerocopy_impl::<Pubkey>()
    };
    const _assert_no_padding: () = {
        if let TypeMeta::Static { size, .. } = <Account as SchemaRead<
            '_,
            ::wincode::config::DefaultConfig,
        >>::TYPE_META {
            if size != core::mem::size_of::<Account>() {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "wincode(assert_zero_copy) was applied to a type with padding",
                        ),
                    );
                };
            }
        } else {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "wincode(assert_zero_copy) was applied to a type with `TypeMeta::Dynamic`",
                    ),
                );
            };
        }
    };
};
impl Account {
    /// Transmute a byte slice into an `Account` reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` is a valid representation of `Account`.
    #[inline(always)]
    pub unsafe fn transmute_unchecked(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != size_of::<Self>() {
            return Err(invalid_account_data_error());
        }
        Ok(&*(bytes.as_ptr() as *const Self))
    }
    /// Transmute a mutable byte slice into a mutable `Account` reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` is a valid representation of `Account`.
    #[inline(always)]
    pub unsafe fn transmute_unchecked_mut(
        bytes: &mut [u8],
    ) -> Result<&mut Self, ProgramError> {
        if bytes.len() != size_of::<Self>() {
            return Err(invalid_account_data_error());
        }
        Ok(&mut *(bytes.as_mut_ptr() as *mut Self))
    }
}
#[cold]
fn invalid_account_data_error() -> ProgramError {
    ProgramError::InvalidAccountData
}
