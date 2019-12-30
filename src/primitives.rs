use std::ops::Deref;

use crate::isolate::Isolate;
use crate::Boolean;
use crate::Local;
use crate::Name;
use crate::Primitive;
use crate::ToLocal;
use crate::Value;

extern "C" {
  fn v8__Null(isolate: *mut Isolate) -> *mut Primitive;

  fn v8__Undefined(isolate: *mut Isolate) -> *mut Primitive;

  fn v8__True(isolate: *mut Isolate) -> *mut Boolean;

  fn v8__False(isolate: *mut Isolate) -> *mut Boolean;
}

pub fn new_null<'sc>(scope: &mut impl ToLocal<'sc>) -> Local<'sc, Primitive> {
  let ptr = unsafe { v8__Null(scope.isolate()) };
  unsafe { scope.to_local(ptr) }.unwrap()
}

pub fn new_undefined<'sc>(
  scope: &mut impl ToLocal<'sc>,
) -> Local<'sc, Primitive> {
  let ptr = unsafe { v8__Undefined(scope.isolate()) };
  unsafe { scope.to_local(ptr) }.unwrap()
}

pub fn new_true<'sc>(scope: &mut impl ToLocal<'sc>) -> Local<'sc, Boolean> {
  let ptr = unsafe { v8__True(scope.isolate()) };
  unsafe { scope.to_local(ptr) }.unwrap()
}

pub fn new_false<'sc>(scope: &mut impl ToLocal<'sc>) -> Local<'sc, Boolean> {
  let ptr = unsafe { v8__False(scope.isolate()) };
  unsafe { scope.to_local(ptr) }.unwrap()
}

impl Deref for Primitive {
  type Target = Value;
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const _ as *const Value) }
  }
}

impl Deref for Boolean {
  type Target = Primitive;
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const _ as *const Primitive) }
  }
}

impl Deref for Name {
  type Target = Primitive;
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const _ as *const Primitive) }
  }
}
