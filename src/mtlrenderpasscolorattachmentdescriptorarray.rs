use objr::bindings::*;
use super::MTLRenderPassColorAttachmentDescriptor;
use foundationr::NSUInteger;
objc_class! {
    pub struct MTLRenderPassColorAttachmentDescriptorArray {
        @class(MTLRenderPassColorAttachmentDescriptorArray)
    }
}
objc_selector_group! {
    pub(crate) trait MTLRenderPassColorAttachmentDescriptorArraySelectors {
        @selector("objectAtIndexedSubscript:")
    }
    impl MTLRenderPassColorAttachmentDescriptorArraySelectors for Sel {}
}


impl MTLRenderPassColorAttachmentDescriptorArray {
    ///Unsafe because if you pass an invalid index, may get an objc exception
    pub unsafe fn get_index(&self, pool: &ActiveAutoreleasePool, index: NSUInteger) -> StrongCell<MTLRenderPassColorAttachmentDescriptor> {
        let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::objectAtIndexedSubscript_(), pool, (index,));
        MTLRenderPassColorAttachmentDescriptor::assume_nonnil(ptr).assume_retained()
    }
}

#[test] fn descriptor_array() {
    autoreleasepool(|pool| {
        let descriptor = super::MTLRenderPassDescriptor::new(pool);
        let array = descriptor.color_attachments(pool);
        let item = unsafe{ array.get_index(pool,0) };
        assert!(item.description(pool).to_str(pool).starts_with("<MTLRenderPassColorAttachmentDescriptor"));
    })


}