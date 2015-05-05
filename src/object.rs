// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::marker::PhantomData;
use glib::object::{Ref, Wrapper};
use glib::type_::StaticType;

pub use glib::object::{Downcast, Upcast};

#[derive(Clone)]
pub struct GtkObject<T>(Ref, PhantomData<T>);

impl<T> Wrapper for GtkObject<T>
where GtkObject<T>: StaticType {
    type GlibType = T;
    #[inline]
    unsafe fn wrap(r: Ref) -> GtkObject<T> { GtkObject(r, PhantomData) }
    #[inline]
    fn as_ref(&self) -> &Ref { &self.0 }
    #[inline]
    fn unwrap(self) -> Ref { self.0 }
}
