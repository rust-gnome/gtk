use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use FixedClass;
use ContainerClass;

pub trait FixedImpl: ContainerImpl + 'static {}

unsafe impl<T: ObjectSubclass + FixedImpl> IsSubclassable<T> for FixedClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
