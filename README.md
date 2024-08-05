# jni-bind-rs
[![Docs](https://docs.rs/jni-bind/badge.svg)](https://docs.rs/jni-bind)
[![Crates.io](https://img.shields.io/crates/v/jni-bind)](https://crates.io/crates/jni-bind)
[![Crates.io](https://img.shields.io/crates/d/jni-bind)](https://crates.io/crates/jni-bind)

Made importing classes from java easy!

This library provides the `import_class` macro that automatically implements the object structure. All the method IDs and classes are cached to increase performance.

# Known Limitations

- cannot import generic classes and methods.
- cannot import constants.

# Examples

Basic usage of `import_class`:

```rust
#![no_main]
#![feature(anonymous_lifetime_in_impl_trait)]

use jni_bind::import_class;
use jni_bind::{jboolean, jint, jlong};

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
    extends Object;
}
```
