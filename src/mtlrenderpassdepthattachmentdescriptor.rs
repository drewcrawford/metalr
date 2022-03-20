use objr::bindings::*;
use super::MTLRenderPassAttachmentDescriptorTrait;
objc_class! {
    pub struct MTLRenderPassDepthAttachmentDescriptor {
        @class(MTLRenderPassDepthAttachmentDescriptor)
    }
}
unsafe impl MTLRenderPassAttachmentDescriptorTrait for MTLRenderPassDepthAttachmentDescriptor {}

objc_selector_group! {
    trait Selectors {
        @selector("clearDepth")
        @selector("setClearDepth:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl MTLRenderPassDepthAttachmentDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe{Self::class().alloc_init(pool).assume_mut() }
    }
    pub fn clearDepth(&self, pool: &ActiveAutoreleasePool) -> f64 {
        unsafe{Self::perform_primitive(self.assume_nonmut_perform(), Sel::clearDepth(), pool, ())}
    }
    pub fn setClearDepth(&mut self, depth: f64, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setClearDepth_(), pool, (depth,))
        }
    }
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let mut descriptor = MTLRenderPassDepthAttachmentDescriptor::new(pool);
        descriptor.setClearDepth(1337.0, pool);
        assert_eq!(descriptor.clearDepth(pool), 1337.0)
    })

}
