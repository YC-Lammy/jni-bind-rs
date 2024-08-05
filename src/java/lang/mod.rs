
use jni::sys::*;
use jni::JNIEnv;

use crate::import_class;

import_class! {
    "java/lang/Object";
    Object;
    constructor();
    /// Indicates whether some other object is "equal to" this one.
    fn equals(&self, other: Object) ->  jboolean;
    /// Returns a hash code value for the object.
    fn hashCode(&self) -> jint;
    /// Wakes up a single thread that is waiting on this object's monitor.
    fn notify(&self) -> ();
    /// Wakes up all threads that are waiting on this object's monitor.
    fn notifyAll(&self) -> ();
    /// Returns a string representation of the object.
    fn toString(&self) -> String;
    /// Causes the current thread to wait until it is awakened,
    /// typically by being notified or interrupted,
    /// or until a certain amount of real time has elapsed.
    fn wait(&self, timeout_millis: jlong, nanos: jint) -> ();
}

impl Object {
    // workaround for generic class object
    pub fn get_class(&self, env: &mut JNIEnv) -> Result<Class, jni::errors::Error> {
        let c = env.get_object_class(self._obj.as_obj())?;

        let r = env.new_global_ref(c)?;
        
        return Ok(Class {
            _obj: r,
        });
    }
}

import_class! {
    "java/lang/Class";
    Class;
    extends Object;
}

import_class!{
    "java/lang/Boolean";
    Boolean;
    extends Object;
    constructor(value: jboolean);

    field f: jboolean;
    
    static fn compare(x: jboolean, y: jboolean) -> jint;
    static fn getBoolean(name: String) -> jboolean;
    static fn hashCode(value: jboolean) -> jint;
    static fn logicalAnd(a: jboolean, b: jboolean) -> jboolean;
    static fn logicalOr(a: jboolean, b: jboolean) -> jboolean;
    static fn logicalXor(a: jboolean, b: jboolean) -> jboolean;
    static fn parseBoolean(s: String) -> jboolean;
    //static fn toString(value: jboolean) -> String;

    fn booleanValue(&self) -> jboolean;
    fn compareTo(&self, b: Boolean) -> jint;
    fn equals(&self, o: Object) -> jboolean;
}

import_class!{
    "java/lang/String";
    String;
}