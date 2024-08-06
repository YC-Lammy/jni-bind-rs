#![no_main]

use jni_bind::{import_class, import_interface};
use jni_bind::{jboolean, jint, jlong, jchar};

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

import_class!{
    "java/lang/String";
    String;
    implements CharSequence;
}

import_interface!{
    "java/lang/CharSequence";
    CharSequence;
    fn charAt(&self, index: jint) -> jchar;
}