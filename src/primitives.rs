
use jni::sys::*;
use jni::objects::JValue;
use jni::JNIEnv;

use crate::{
    IsA,
    JReturnType,
    JBindingType
};


unsafe impl JBindingType for jboolean {
    const SIGNATURE: &'static str = "Z";
    const NAME: &'static str = "boolean";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { z: *self }
    }
    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Bool(*self)
    }
}

unsafe impl IsA<jboolean> for jboolean{
    unsafe fn as_ref(&self) -> &jboolean {
        self
    }
}

unsafe impl JReturnType for jboolean {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Boolean);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.z
    }
}

unsafe impl JBindingType for jbyte {
    const SIGNATURE: &'static str = "B";
    const NAME: &'static str = "byte";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { b: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Byte(*self)
    }
}

unsafe impl IsA<Self> for jbyte{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jbyte {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Byte);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.b
    }
}

unsafe impl JBindingType for jchar {
    const SIGNATURE: &'static str = "C";
    const NAME: &'static str = "char";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { c: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Char(*self)
    }
}

unsafe impl IsA<Self> for jchar{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jchar {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Char);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.c
    }
}

unsafe impl JBindingType for jshort {
    const SIGNATURE: &'static str = "S";
    const NAME: &'static str = "short";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { s: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Short(*self)
    }
}

unsafe impl IsA<Self> for jshort{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jshort {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Short);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.s
    }
}

unsafe impl JBindingType for jint {
    const SIGNATURE: &'static str = "I";
    const NAME: &'static str = "int";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { i: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Int(*self)
    }
}

unsafe impl IsA<Self> for jint{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jint {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Int);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.i
    }
}

unsafe impl JBindingType for jlong {
    const SIGNATURE: &'static str = "J";
    const NAME: &'static str = "long";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { j: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Long(*self)
    }
}

unsafe impl IsA<Self> for jlong{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jlong {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Long);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.j
    }
}

unsafe impl JBindingType for jfloat {
    const SIGNATURE: &'static str = "F";
    const NAME: &'static str = "float";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { f: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Float(*self)
    }
}

unsafe impl IsA<Self> for jfloat{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jfloat {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Float);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.f
    }
}

unsafe impl JBindingType for jdouble {
    const SIGNATURE: &'static str = "D";
    const NAME: &'static str = "double";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue { d: *self }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Double(*self)
    }
}

unsafe impl IsA<Self> for jdouble{
    unsafe fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl JReturnType for jdouble {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType =
        jni::signature::ReturnType::Primitive(jni::signature::Primitive::Double);

    unsafe fn from_jvalue(_env: &mut JNIEnv, value: jvalue) -> Self {
        value.d
    }
}

pub struct JByteArray {
    _obj: jni::objects::GlobalRef,
}

unsafe impl JBindingType for JByteArray {
    const SIGNATURE: &'static str = "[B";
    const NAME: &'static str = "byte[]";

    unsafe fn to_jvalue(&self) -> jvalue {
        jvalue {
            l: self._obj.as_raw(),
        }
    }

    unsafe fn to_jvalue_ref<'obj_ref>(&'obj_ref self) -> JValue<'_, 'obj_ref> {
        JValue::Object(&self._obj)
    }
}

unsafe impl JReturnType for JByteArray {
    const SIGNATURE: &'static str = <Self as JBindingType>::SIGNATURE;
    const NAME: &'static str = <Self as JBindingType>::NAME;
    const JNI_RETURN_TY: jni::signature::ReturnType = jni::signature::ReturnType::Array;

    unsafe fn from_jvalue(env: &mut JNIEnv, value: jvalue) -> Self {
        let o = jni::objects::JByteArray::from_raw(value.l);
        let r = env.new_global_ref(o).expect("failed to create global ref");

        JByteArray {
            _obj: r,
        }
    }
}