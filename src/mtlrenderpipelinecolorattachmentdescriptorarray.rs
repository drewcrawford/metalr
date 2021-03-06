use objr::bindings::*;
use crate::MTLRenderPipelineColorAttachmentDescriptor;
use foundationr::NSUInteger;
objc_class! {
    pub struct MTLRenderPipelineColorAttachmentDescriptorArray {
        @class(MTLRenderPipelineColorAttachmentDescriptorArray)
    }
}

#[allow(non_snake_case)]
impl MTLRenderPipelineColorAttachmentDescriptorArray {
    ///Unsafe because you might access an array OOB, which causes an objc exception and
    /// therefore UB
    pub unsafe fn objectAtIndexedSubscript(&self,subscript: NSUInteger,pool: &ActiveAutoreleasePool) -> StrongMutCell<MTLRenderPipelineColorAttachmentDescriptor> {
        use crate::mtlrenderpasscolorattachmentdescriptorarray::MTLRenderPassColorAttachmentDescriptorArraySelectors;
        let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::objectAtIndexedSubscript_(), pool, (subscript,));
        MTLRenderPipelineColorAttachmentDescriptor::assume_nonnil(ptr).assume_retained().assume_mut()
    }
}

#[test] fn smoke_test() {
    autoreleasepool(|pool| {
        let o = MTLRenderPipelineColorAttachmentDescriptorArray::class().alloc_init(pool);
        let attachment = unsafe{ o.objectAtIndexedSubscript( 0,pool) };
        println!("{}",attachment);
    })
}