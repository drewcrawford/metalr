use objr::bindings::*;
use super::MTLRenderPassColorAttachmentDescriptorArray;
use foundationr::NSUInteger;
objc_class! {
    pub struct MTLRenderPassDescriptor {
        @class(MTLRenderPassDescriptor)
    }
}
objc_selector_group! {
    trait MTLRenderPassDescriptorSelectors {
        @selector("setRenderTargetHeight:")
        @selector("setRenderTargetWidth:")
    }
    impl MTLRenderPassDescriptorTrait for Sel {}
}
#[allow(non_snake_case)]
impl MTLRenderPassDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<MTLRenderPassDescriptor> {
        unsafe{ Self::class().alloc_init(pool).assume_mut() }
    }
    pub fn set_renderTargetHeight(&mut self, pool: &ActiveAutoreleasePool, height: NSUInteger) {
        unsafe { Self::perform_primitive(self, Sel::setRenderTargetHeight_(),pool, (height,)) }
    }
    pub fn set_renderTargetWidth(&mut self, pool: &ActiveAutoreleasePool, width: NSUInteger) {
        unsafe { Self::perform_primitive(self,Sel::setRenderTargetWidth_(), pool, (width,)) }
    }
    pub fn color_attachments<'a>(&self, pool: &ActiveAutoreleasePool) -> StrongCell<MTLRenderPassColorAttachmentDescriptorArray> {
        //we borrow the selector we already declared!
        use crate::mtlrenderpipelinedescriptor::MTLRenderPipelineDescriptorSelectors;
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::colorAttachments(), pool, ());
            MTLRenderPassColorAttachmentDescriptorArray::assume_nonnil(ptr).assume_retained()
        }
    }
}

#[test]
fn configure_target() {
    autoreleasepool(|pool| {
        let mut descriptor = MTLRenderPassDescriptor::new(pool);
        descriptor.set_renderTargetHeight(pool,1000);
        descriptor.set_renderTargetWidth(pool,500);
        let description_strong = descriptor.description(pool);
        let description = description_strong.to_str(pool);
        assert!(description.contains("renderTargetHeight = 1000"));
        assert!(description.contains("renderTargetWidth = 500"));

    });

}