#![feature(anonymous_lifetime_in_impl_trait)]

use jni::objects::JValue;
use jni::sys::*;

pub use jni;
pub use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
pub use jni::JNIEnv;

pub mod export {
    pub use const_format;
    pub use paste;
}

mod primitives;
pub mod java;

/// this trait should only be implemented by macro.
/// Manually implementing this trait may cause undefined behaviour
pub unsafe trait IsA<T>{
    unsafe fn as_ref(&self) -> &T;
}

pub unsafe trait JReturnType {
    const SIGNATURE: &'static str;
    const NAME: &'static str;
    const JNI_RETURN_TY: jni::signature::ReturnType;

    unsafe fn from_jvalue(env: &mut JNIEnv, value: jvalue) -> Self;
}

unsafe impl JReturnType for () {
    const SIGNATURE: &'static str = "V";
    const NAME: &'static str = "void";
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Void);

    unsafe fn from_jvalue(_env: &mut JNIEnv, _value: jvalue) -> Self {
        ()
    }
}

/// this trait should only be implemented by macro,
/// manually implement this trait will cause undefined behaviour
pub unsafe trait JBindingType {
    const SIGNATURE: &'static str;
    const NAME: &'static str;

    unsafe fn to_jvalue(&self) -> jvalue;
    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref>;
}

#[macro_export]
macro_rules! import_class {
    (
        $sig: expr;
        $name: ident;
        $(extends $($parent_class: ident),+;)?
        $(implements $($parent_interface: ident),+;)?
        $(
            constructor ($($ctor_arg:ident : $ctor_arg_ty:ty),*);
        )?
        $(
            field $field:ident : $field_ty:ty;
        )*
        $(
            static fn $static_method:ident ($($static_arg:ident : $static_arg_ty:ty),*) -> $static_ret:ty;
        )*
        $(
            $(#[doc=$doc:expr])*
            fn $method:ident (&self $(, $arg:ident : $arg_ty:ty)*) -> $ret:ty;
        )*
    ) => {
        #[repr(transparent)]
        #[derive(Debug, Clone)]
        pub struct $name{
            _obj: $crate::jni::objects::GlobalRef,
        }

        unsafe impl $crate::JBindingType for $name {
            const SIGNATURE: &'static str = concat!("L", $sig, ";");
            const NAME: &'static str = $sig;

            unsafe fn to_jvalue(&self) -> $crate::jni::sys::jvalue {
                $crate::jni::sys::jvalue{
                    l: self._obj.as_obj().as_raw()
                }
            }

            unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> $crate::jni::objects::JValue<'_, 'obj_ref>{
                $crate::jni::objects::JValue::Object(
                    self._obj.as_obj()
                )
            }
        }

        unsafe impl $crate::JReturnType for $name {
            const SIGNATURE: &'static str = <Self as $crate::JBindingType>::SIGNATURE;
            const NAME: &'static str = <Self as $crate::JBindingType>::NAME;
            const JNI_RETURN_TY: jni::signature::ReturnType = jni::signature::ReturnType::Object;

            unsafe fn from_jvalue(env: &mut$crate::JNIEnv, value: $crate::jni::sys::jvalue) -> Self {
                let o = $crate::jni::objects::JObject::from_raw(value.l);
                let r = env.new_global_ref(o).expect("failed to create global ref");
                Self {
                    _obj: r,
                }
            }
        }
        
        unsafe impl $crate::IsA<$name> for $name{
            unsafe fn as_ref(&self) -> &$name{
                self
            }
        }

        unsafe impl $crate::IsA<$name> for &$name{
            unsafe fn as_ref(&self) -> &$name{
                self
            }
        }
        
        $(
            $(
                impl ::core::convert::AsRef<$parent_class> for $name{
                    fn as_ref(&self) -> &$parent_class{
                        unsafe{
                            core::mem::transmute(self)
                        }
                    }
                }

                impl From<$name> for $parent_class{
                    fn from(value: $name) -> $parent_class{
                        unsafe{
                            core::mem::transmute(value)
                        }
                    }
                }

                unsafe impl $crate::IsA<$parent_class> for $name{
                    unsafe fn as_ref(&self) -> &$parent_class{
                        ::core::convert::AsRef::as_ref(self)
                    }
                }
        
                unsafe impl $crate::IsA<$parent_class> for &$name{
                    unsafe fn as_ref(&self) -> &$parent_class{
                        ::core::convert::AsRef::as_ref(self)
                    }
                }
            )*
        )?

        #[allow(unused)]
        impl $name {
            #[allow(dead_code)]
            fn class<'local>(env: &mut $crate::jni::JNIEnv<'local>) -> Result<$crate::jni::objects::JClass<'local>, $crate::jni::errors::Error>{
                static CACHE: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

                let id = CACHE.load(core::sync::atomic::Ordering::Relaxed);

                let mut env_ptr = id >> 32;
                let mut class_ptr = id & 0xFFFFFFFF;

                if env_ptr == 0 || env_ptr != env.get_raw() as u64 {
                    env_ptr = env.get_raw() as u64;

                    let class = env
                        .find_class(<Self as $crate::JBindingType>::NAME)?;

                    class_ptr = class.as_raw() as u64;

                    let cache = (env_ptr << 32) | class_ptr;

                    CACHE.store(cache, core::sync::atomic::Ordering::Relaxed);
                };

                unsafe{
                    return Ok($crate::jni::objects::JClass::from_raw(class_ptr as _))
                }
            }

            $(
                pub fn new(env: &mut $crate::jni::JNIEnv $(, $ctor_arg : impl $crate::IsA<$ctor_arg_ty>)*) -> Result<Self, $crate::jni::errors::Error> {
                    let class = Self::class(env)?;

                    const CTOR_SIG: &str = $crate::export::const_format::concatcp!(
                        "(",
                        $(
                            <$ctor_arg_ty as $crate::JBindingType>::SIGNATURE,
                        )*
                        ")V"
                    );

                    static CACHE: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

                    let id = CACHE.load(core::sync::atomic::Ordering::Relaxed);

                    let mut env_ptr = id >> 32;
                    let mut method_id = id & 0xFFFFFFFF;

                    if env_ptr == 0 || env_ptr != env.get_raw() as u64 {
                        env_ptr = env.get_raw() as u64;

                        method_id = env.get_method_id(
                            &class,
                            "<init>",
                            CTOR_SIG
                        )?.into_raw() as u64;

                        let cache = (env_ptr << 32) | method_id;

                        CACHE.store(cache, core::sync::atomic::Ordering::Relaxed);
                    };

                    let obj = unsafe{env.new_object_unchecked(
                        class,
                        $crate::jni::objects::JMethodID::from_raw(method_id as _),
                        &[
                            $(
                                <$ctor_arg_ty as $crate::JBindingType>::to_jvalue(unsafe{$crate::IsA::<$ctor_arg_ty>::as_ref(&$ctor_arg)})
                            ),*
                        ]
                    )?};

                    let r = env.new_global_ref(obj)?;

                    return Ok(Self {
                        _obj: r,
                    });
                }
            )?


            $(
                $crate::export::paste::paste!{
                    pub fn [<get_ $field:snake>](&self, env: &mut $crate::jni::JNIEnv) -> Result<$field_ty, $crate::jni::errors::Error>{
                        let class = Self::class(env)?;

                        static CACHE: ::core::sync::atomic::AtomicU64 = ::core::sync::atomic::AtomicU64::new(0);

                        let id = CACHE.load(::core::sync::atomic::Ordering::Relaxed);

                        let mut env_ptr = id >> 32;
                        let mut field_id = id & 0xFFFFFFFF;

                        if env_ptr == 0 || env_ptr != env.get_raw() as u64 {
                            env_ptr = env.get_raw() as u64;

                            field_id = env.get_field_id(
                                &class,
                                stringify!(u),
                                <i8 as $crate::JReturnType>::SIGNATURE
                            )?.into_raw() as u64;

                            let cache = (env_ptr << 32) | field_id;

                            CACHE.store(cache, ::core::sync::atomic::Ordering::Relaxed);
                        };

                        unsafe{
                            let b = env.get_field_unchecked(
                                self._obj.as_obj(),
                                $crate::jni::objects::JFieldID::from_raw(field_id as _),
                                <$field_ty as crate::JReturnType>::JNI_RETURN_TY
                            )?;

                            return Ok(<$field_ty as $crate::JReturnType>::from_jvalue(env, b.as_jni()))
                        }
                    }

                    pub fn [<set_ $field:snake>](&self, env: &mut $crate::jni::JNIEnv, value: $field_ty) -> Result<(), crate::jni::errors::Error>{
                        let class = Self::class(env)?;

                        static CACHE: ::core::sync::atomic::AtomicU64 = ::core::sync::atomic::AtomicU64::new(0);

                        let id = CACHE.load(::core::sync::atomic::Ordering::Relaxed);

                        let mut env_ptr = id >> 32;
                        let mut field_id = id & 0xFFFFFFFF;

                        if env_ptr == 0 || env_ptr != env.get_raw() as u64 {
                            env_ptr = env.get_raw() as u64;

                            field_id = env.get_field_id(
                                &class,
                                stringify!(u),
                                <$field_ty as $crate::JReturnType>::SIGNATURE
                            )?.into_raw() as u64;

                            let cache = (env_ptr << 32) | field_id;

                            CACHE.store(cache, ::core::sync::atomic::Ordering::Relaxed);
                        };

                        unsafe{
                            env.set_field_unchecked(
                                self._obj.as_obj(),
                                $crate::jni::objects::JFieldID::from_raw(field_id as _),
                                <$field_ty as $crate::JBindingType>::to_jvalue_ref(&value)
                            )?;

                            return Ok(())
                        }
                    }
                }
            )*

            $(
                $crate::export::paste::paste!{
                    pub fn [<$static_method:snake>](env: &mut $crate::jni::JNIEnv $(, $static_arg : impl $crate::IsA<$static_arg_ty>)*) -> Result<$static_ret, $crate::jni::errors::Error>{
                        let class = Self::class(env)?;

                        const METHOD_SIG: &str = $crate::export::const_format::concatcp!(
                            "(",
                            $(
                                <$static_arg_ty as $crate::JBindingType>::SIGNATURE,
                            )*
                            ")",
                            <$static_ret as $crate::JReturnType>::SIGNATURE
                        );

                        static CACHE: ::core::sync::atomic::AtomicU64 = ::core::sync::atomic::AtomicU64::new(0);

                        let id = CACHE.load(::core::sync::atomic::Ordering::Relaxed);

                        let mut env_ptr = id >> 32;
                        let mut method_id = id & 0xFFFFFFFF;

                        if env_ptr == 0 || env_ptr != env.get_raw() as u64 {
                            env_ptr = env.get_raw() as u64;

                            method_id = env.get_static_method_id(
                                &class,
                                stringify!($static_method),
                                METHOD_SIG
                            )?.into_raw() as u64;

                            let cache = (env_ptr << 32) | method_id;

                            CACHE.store(cache, core::sync::atomic::Ordering::Relaxed);
                        };

                        unsafe{
                            let re = env.call_static_method_unchecked(
                                &class,
                                $crate::jni::objects::JStaticMethodID::from_raw(method_id as _),
                                <$static_ret as $crate::JReturnType>::JNI_RETURN_TY,
                                &[
                                    $(
                                        <$static_arg_ty as $crate::JBindingType>::to_jvalue(unsafe{$crate::IsA::<$static_arg_ty>::as_ref(&$static_arg)})
                                    ),*
                                ]
                            )?;

                            return Ok(<$static_ret as $crate::JReturnType>::from_jvalue(env, re.as_jni()))
                        };
                    }
                }
            )*

            $(
                $crate::export::paste::paste!{
                    $(#[doc=$doc])*
                    pub fn [<$method:snake>](&self, env: &mut $crate::jni::JNIEnv $(, $arg : impl $crate::IsA<$arg_ty>)*) -> Result<$ret, $crate::jni::errors::Error>{
                        let class = Self::class(env)?;

                        const METHOD_SIG: &str = $crate::export::const_format::concatcp!(
                            "(",
                            $(
                                <$arg_ty as $crate::JBindingType>::SIGNATURE,
                            )*
                            ")",
                            <$ret as $crate::JReturnType>::SIGNATURE
                        );

                        static CACHE: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

                        let id = CACHE.load(core::sync::atomic::Ordering::Relaxed);

                        let mut env_ptr = id >> 32;
                        let mut method_id = id & 0xFFFFFFFF;

                        if env_ptr == 0 || env_ptr != env.get_raw() as u64 {
                            env_ptr = env.get_raw() as u64;

                            method_id = env.get_method_id(
                                &class,
                                stringify!($method),
                                METHOD_SIG
                            )?.into_raw() as u64;

                            let cache = (env_ptr << 32) | method_id;

                            CACHE.store(cache, core::sync::atomic::Ordering::Relaxed);
                        };

                        unsafe{
                            let r = env.call_method_unchecked(
                                self._obj.as_obj(),
                                $crate::jni::objects::JMethodID::from_raw(method_id as _),
                                <$ret as $crate::JReturnType>::JNI_RETURN_TY,
                                &[
                                    $(
                                        <$arg_ty as $crate::JBindingType>::to_jvalue(unsafe{$crate::IsA::<$arg_ty>::as_ref(&$arg)})
                                    ),*
                                ]
                            )?;

                            return Ok(<$ret as $crate::JReturnType>::from_jvalue(env, r.as_jni()))
                        };
                    }
                }
            )*
        }

    };
}
