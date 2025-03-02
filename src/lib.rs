#![allow(clippy::ptr_eq)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::borrow::Borrow;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::sync::Arc;

#[derive(Clone, Copy, Default)]
pub struct ByAddr<T>(pub T)
where
   T: ?Sized + Deref;

impl<T> From<T> for ByAddr<T>
where
   T: Deref,
{
   fn from(t: T) -> ByAddr<T> {
      ByAddr(t)
   }
}

impl<T> ByAddr<T>
where
   T: ?Sized + Deref,
{
   fn addr(&self) -> *const T::Target {
      &*self.0
   }

   pub fn from_ref(r: &T) -> &Self {
      unsafe { &*(r as *const T as *const Self) }
   }
}

impl<T> PartialEq for ByAddr<T>
where
   T: ?Sized + Deref,
{
   fn eq(&self, other: &Self) -> bool {
      self.addr() as *const () == other.addr() as *const ()
   }
}

impl<T> Eq for ByAddr<T> where T: ?Sized + Deref {}

impl<T> PartialOrd for ByAddr<T>
where
   T: ?Sized + Deref,
{
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some((self.addr() as *const ()).cmp(&(other.addr() as *const ())))
   }
}

impl<T> Ord for ByAddr<T>
where
   T: ?Sized + Deref,
{
   fn cmp(&self, other: &Self) -> Ordering {
      (self.addr() as *const ()).cmp(&(other.addr() as *const ()))
   }
}

impl<T> Hash for ByAddr<T>
where
   T: ?Sized + Deref,
{
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.addr().hash(state)
   }
}

impl<T> Deref for ByAddr<T>
where
   T: ?Sized + Deref,
{
   type Target = T;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl<T> DerefMut for ByAddr<T>
where
   T: ?Sized + Deref,
{
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
   }
}

impl<T, U> AsRef<U> for ByAddr<T>
where
   T: ?Sized + Deref + AsRef<U>,
{
   fn as_ref(&self) -> &U {
      self.0.as_ref()
   }
}

impl<T, U> AsMut<U> for ByAddr<T>
where
   T: ?Sized + Deref + AsMut<U>,
{
   fn as_mut(&mut self) -> &mut U {
      self.0.as_mut()
   }
}

#[cfg(feature = "std")]
impl<'a, T> Borrow<ByAddr<&'a T>> for ByAddr<Box<T>> {
   fn borrow(&self) -> &ByAddr<&'a T> {
      unsafe { &*(self.addr() as *const ByAddr<&'a T>) }
   }
}

#[cfg(feature = "std")]
impl<'a, T> Borrow<ByAddr<&'a T>> for ByAddr<Rc<T>> {
   fn borrow(&self) -> &ByAddr<&'a T> {
      unsafe { &*(self.addr() as *const ByAddr<&'a T>) }
   }
}

#[cfg(feature = "std")]
impl<'a, T> Borrow<ByAddr<&'a T>> for ByAddr<Arc<T>> {
   fn borrow(&self) -> &ByAddr<&'a T> {
      unsafe { &*(self.addr() as *const ByAddr<&'a T>) }
   }
}
