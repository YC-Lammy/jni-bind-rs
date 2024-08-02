#![feature(anonymous_lifetime_in_impl_trait)]

use std::marker::PhantomData;

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

#[derive(Debug, Clone)]
pub struct GlobalRef<T: JBindingType<'static>> where for<'a> &'a T : JBindingRef<'a, T>{
    _ref: jni::objects::GlobalRef,
    _mark: PhantomData<T>,
}

impl<T> GlobalRef<T>  where T: JBindingType<'static>, for<'a> &'a T : JBindingRef<'a, T>{
    pub unsafe fn from_jni(global_ref: jni::objects::GlobalRef) -> Self {
        Self {
            _ref: global_ref,
            _mark: PhantomData,
        }
    }
    pub fn as_jni(&self) -> &jni::objects::GlobalRef{
        &self._ref
    }
    pub fn into_jni(self) -> jni::objects::GlobalRef{
        self._ref
    }
}

impl<T> core::ops::Deref for GlobalRef<T>   where T: JBindingType<'static>, for<'a> &'a T : JBindingRef<'a, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe{
            core::mem::transmute(self.as_jni().as_obj())
        }
    }
}

/// this trait should only be implemented by macro.
/// Manually implementing this trait may cause undefined behaviour
pub unsafe trait JBindingRef<'local, T>{
    unsafe fn as_ref(&self) -> &T;
}

pub unsafe trait JReturnType<'local> {
    const SIGNATURE: &'static str;
    const NAME: &'static str;
    const JNI_RETURN_TY: jni::signature::ReturnType;

    unsafe fn from_jvalue(value: jvalue) -> Self;
}

unsafe impl JReturnType<'static> for () {
    const SIGNATURE: &'static str = "V";
    const NAME: &'static str = "void";
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Void);

    unsafe fn from_jvalue(_value: jvalue) -> Self {
        ()
    }
}

/// this trait should only be implemented by macro,
/// manually implement this trait will cause undefined behaviour
pub unsafe trait JBindingType<'local> {
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
        #[derive(Debug)]
        pub struct $name<'local> {
            _obj: $crate::jni::sys::jobject,
            _mark: ::core::marker::PhantomData<&'local ()>
        }

        unsafe impl<'local> $crate::JBindingType<'local> for $name<'local> {
            const SIGNATURE: &'static str = concat!("L", $sig, ";");
            const NAME: &'static str = $sig;

            unsafe fn to_jvalue(&self) -> $crate::jni::sys::jvalue {
                $crate::jni::sys::jvalue{
                    l: self._obj
                }
            }

            unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> $crate::jni::objects::JValue<'_, 'obj_ref>{
                $crate::jni::objects::JValue::Object(
                    unsafe{
                        core::mem::transmute(self)
                    }
                )
            }
        }

        unsafe impl<'local> $crate::JReturnType<'local> for $name<'local> {
            const SIGNATURE: &'static str = <Self as $crate::JBindingType>::SIGNATURE;
            const NAME: &'static str = <Self as $crate::JBindingType>::NAME;
            const JNI_RETURN_TY: jni::signature::ReturnType = jni::signature::ReturnType::Object;

            unsafe fn from_jvalue(value: $crate::jni::sys::jvalue) -> Self {
                Self {
                    _obj: value.l,
                    _mark: ::core::marker::PhantomData
                }
            }
        }
        
        unsafe impl<'local> $crate::JBindingRef<'local, $name<'local>> for $name<'local>{
            unsafe fn as_ref(&self) -> &$name<'local>{
                self
            }
        }

        unsafe impl<'a, 'local> $crate::JBindingRef<'a, $name<'local>> for &'a $name<'local>{
            unsafe fn as_ref(&self) -> &$name<'local>{
                self
            }
        }
        
        impl ::core::convert::AsRef<$name<'static>> for $crate::GlobalRef<$name<'static>>{
            fn as_ref(&self) -> &$name<'static>{
                unsafe{
                    core::mem::transmute(self.as_jni().as_obj())
                }
            }
        }

        $(
            $(
                impl<'local> ::core::convert::AsRef<$parent_class<'local>> for $name<'local>{
                    fn as_ref(&self) -> &$parent_class<'local>{
                        unsafe{
                            core::mem::transmute(self)
                        }
                    }
                }

                impl<'local> From<$name<'local>> for $parent_class<'local>{
                    fn from(value: $name<'local>) -> $parent_class<'local>{
                        unsafe{
                            core::mem::transmute(value)
                        }
                    }
                }

                unsafe impl<'local> $crate::JBindingRef<'local, $parent_class<'local>> for $name<'local>{
                    unsafe fn as_ref(&self) -> &$parent_class<'local>{
                        ::core::convert::AsRef::as_ref(self)
                    }
                }
        
                unsafe impl<'a, 'local> $crate::JBindingRef<'a, $parent_class<'local>> for &'a $name<'local>{
                    unsafe fn as_ref(&self) -> &$parent_class<'local>{
                        ::core::convert::AsRef::as_ref(self)
                    }
                }
            )*
        )?

        #[allow(unused)]
        impl<'local> $name<'local> {
            #[allow(dead_code)]
            fn class(env: &mut $crate::jni::JNIEnv<'local>) -> Result<$crate::jni::objects::JClass<'local>, $crate::jni::errors::Error>{
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

            pub fn create_global_ref(self, env: &mut $crate::jni::JNIEnv<'local>) -> Result<$crate::GlobalRef<$name<'static>>, $crate::jni::errors::Error>{
                let global_ref = env.new_global_ref(
                    unsafe{
                        $crate::jni::objects::JObject::from_raw(self._obj)
                    }
                )?;

                return Ok(
                    unsafe{$crate::GlobalRef::from_jni(global_ref)}
                )
            }

            $(
                pub fn new(env: &mut $crate::jni::JNIEnv<'local> $(, $ctor_arg : impl $crate::JBindingRef<'local, $ctor_arg_ty>)*) -> Result<Self, $crate::jni::errors::Error> {
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
                                <$ctor_arg_ty as $crate::JBindingType>::to_jvalue(unsafe{$crate::JBindingRef::<$ctor_arg_ty>::as_ref(&$ctor_arg)})
                            ),*
                        ]
                    )?};

                    return Ok(Self {
                        _obj: obj.as_raw(),
                        _mark: ::core::marker::PhantomData
                    });
                }
            )?


            $(
                $crate::export::paste::paste!{
                    pub fn [<get_ $field:snake>](&self, env: &mut $crate::jni::JNIEnv<'local>) -> Result<$field_ty, $crate::jni::errors::Error>{
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
                                &self._obj,
                                $crate::jni::objects::JFieldID::from_raw(field_id as _),
                                <$field_ty as crate::JReturnType>::JNI_RETURN_TY
                            )?;

                            return Ok(<$field_ty as JReturnType>::from_jvalue(b.as_jni()))
                        }
                    }

                    pub fn [<set_ $field:snake>](&self, env: &mut $crate::jni::JNIEnv<'local>, value: $field_ty) -> Result<(), crate::jni::errors::Error>{
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
                                &self._obj,
                                $crate::jni::objects::JFieldID::from_raw(field_id as _),
                                <$field_ty as JBindingType>::to_jvalue_ref(&value)
                            )?;

                            return Ok(())
                        }
                    }
                }
            )*

            $(
                $crate::export::paste::paste!{
                    pub fn [<$static_method:snake>](env: &mut $crate::jni::JNIEnv<'local> $(, $static_arg : impl $crate::JBindingRef<'local, $static_arg_ty>)*) -> Result<$static_ret, $crate::jni::errors::Error> where $static_ret: $crate::JReturnType<'local>{
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
                            let r = env.call_static_method_unchecked(
                                &class,
                                $crate::jni::objects::JStaticMethodID::from_raw(method_id as _),
                                <$static_ret as $crate::JReturnType>::JNI_RETURN_TY,
                                &[
                                    $(
                                        <$static_arg_ty as $crate::JBindingType>::to_jvalue(unsafe{$crate::JBindingRef::<$static_arg_ty>::as_ref(&$static_arg)})
                                    ),*
                                ]
                            )?;

                            return Ok(<$static_ret as $crate::JReturnType>::from_jvalue(r.as_jni()))
                        };
                    }
                }
            )*

            $(
                $crate::export::paste::paste!{
                    $(#[doc=$doc])*
                    pub fn [<$method:snake>](&self, env: &mut $crate::jni::JNIEnv<'local> $(, $arg : impl $crate::JBindingRef<'local, $arg_ty>)*) -> Result<$ret, $crate::jni::errors::Error>{
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
                                $crate::jni::objects::JObject::from_raw(self._obj),
                                $crate::jni::objects::JMethodID::from_raw(method_id as _),
                                <$ret as $crate::JReturnType>::JNI_RETURN_TY,
                                &[
                                    $(
                                        <$arg_ty as $crate::JBindingType>::to_jvalue(unsafe{$crate::JBindingRef::<$arg_ty>::as_ref(&$arg)})
                                    ),*
                                ]
                            )?;

                            return Ok(<$ret as $crate::JReturnType>::from_jvalue(r.as_jni()))
                        };
                    }
                }
            )*
        }

    };
}
