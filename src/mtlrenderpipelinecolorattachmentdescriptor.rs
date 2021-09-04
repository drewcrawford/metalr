use objr::bindings::*;
use crate::MTLPixelFormat;

objc_class! {
    pub struct MTLRenderPipelineColorAttachmentDescriptor {
        @class(MTLRenderPipelineColorAttachmentDescriptor)
    }
}
#[allow(non_snake_case)]
impl MTLRenderPipelineColorAttachmentDescriptor {
    pub fn set_pixelFormat(&mut self, pool: &ActiveAutoreleasePool, pixelFormat: MTLPixelFormat) {
        unsafe {
            use crate::mtltexturedescriptor::MTLTextureDescriptorSelectors;
            Self::perform_primitive(self, Sel::setPixelFormat_(), pool, (pixelFormat.field(),))
        }
    }
}

#[test] fn smoke_test() {
    autoreleasepool(|pool| {
        let mut d = unsafe{ MTLRenderPipelineColorAttachmentDescriptor::class().alloc_init(pool).assume_mut() };
        d.set_pixelFormat(pool, MTLPixelFormat::A8Unorm);
        let description = d.description(pool).to_str(pool).to_owned();
        assert!(description.contains("MTLPixelFormatA8Unorm"));
    })
}