/*!mtlsampler.h */

use objr::{objc_class, objc_instance};
use objr::bindings::{ActiveAutoreleasePool, StrongMutCell};
objc_class! {
    pub struct MTLSamplerDescriptor {
        @class(MTLSamplerDescriptor)
    }
}

impl MTLSamplerDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        use objr::bindings::ObjcClass;
        unsafe{MTLSamplerDescriptor::class().alloc_init(pool).assume_mut() }
    }
}

objc_instance! {
    pub struct MTLSamplerState;
}
//I mean, it can be sent to the GPU, so.
unsafe impl Send for MTLSamplerState {}