use objr::bindings::*;
use super::MTLRenderPassColorAttachmentDescriptorArray;
use crate::MTLRenderPassDepthAttachmentDescriptor;
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
        @selector("depthAttachment")
        @selector("setDepthAttachment:")
    }
    impl MTLRenderPassDescriptorTrait for Sel {}
}
#[allow(non_snake_case)]
impl MTLRenderPassDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<MTLRenderPassDescriptor> {
        unsafe{ Self::class().alloc_init(pool).assume_mut() }
    }
    pub fn set_renderTargetHeight(&mut self,  height: NSUInteger,pool: &ActiveAutoreleasePool) {
        unsafe { Self::perform_primitive(self, Sel::setRenderTargetHeight_(),pool, (height,)) }
    }
    pub fn set_renderTargetWidth(&mut self,  width: NSUInteger,pool: &ActiveAutoreleasePool) {
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
    pub fn depthAttachment(&self, pool: &ActiveAutoreleasePool) -> StrongCell<MTLRenderPassDepthAttachmentDescriptor> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::depthAttachment(), pool, ());
            MTLRenderPassDepthAttachmentDescriptor::assume_nonnil(raw) //null_resettable property
                .assume_retained()
        }
    }
    pub fn set_depthAttachment(&mut self, attachment: Option<&MTLRenderPassDepthAttachmentDescriptor>, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDepthAttachment_(), pool, (attachment.as_ptr(),))
        }
    }
}

#[test]
fn configure_target() {
    autoreleasepool(|pool| {
        let mut descriptor = MTLRenderPassDescriptor::new(pool);
        descriptor.set_renderTargetHeight(1000,pool);
        descriptor.set_renderTargetWidth(500,pool);
        let description_strong = descriptor.description(pool);
        let description = description_strong.to_str(pool);
        assert!(description.contains("renderTargetHeight = 1000"));
        assert!(description.contains("renderTargetWidth = 500"));

        let depth_descriptor = MTLRenderPassDepthAttachmentDescriptor::new(pool);
        descriptor.set_depthAttachment(Some(&depth_descriptor), pool);
    });

}