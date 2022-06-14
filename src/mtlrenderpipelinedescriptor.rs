use objr::bindings::*;
use crate::{MTLFunction, MTLPixelFormat, MTLRenderPipelineColorAttachmentDescriptorArray};
objc_class! {
    pub struct MTLRenderPipelineDescriptor {
        @class(MTLRenderPipelineDescriptor)
    }
}
objc_selector_group! {
    pub(crate) trait MTLRenderPipelineDescriptorSelectors {
        @selector("setVertexFunction:")
        @selector("setFragmentFunction:")
        @selector("colorAttachments")
        @selector("depthAttachmentPixelFormat")
        @selector("setDepthAttachmentPixelFormat:")

    }
    impl MTLRenderPipelineDescriptorSelectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLRenderPipelineDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe{ MTLRenderPipelineDescriptor::class().alloc_init(pool).assume_mut() }
    }
    pub fn set_vertex_function(&mut self, function: &MTLFunction,pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setVertexFunction_(), pool, (function.assume_nonmut_perform(),))
        }
    }
    pub fn set_fragment_function(&mut self,  function: &MTLFunction,pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setFragmentFunction_(), pool, (function.assume_nonmut_perform(),))
        }
    }
    pub fn colorAttachments(&mut self, pool: &ActiveAutoreleasePool) -> StrongCell<MTLRenderPipelineColorAttachmentDescriptorArray> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::colorAttachments(), pool, ());
            MTLRenderPipelineColorAttachmentDescriptorArray::assume_nonnil(ptr).assume_retained()
        }
    }
    pub fn depthAttachmentPixelFormat(&self, pool: &ActiveAutoreleasePool) -> MTLPixelFormat {
        unsafe {
            MTLPixelFormat(Self::perform_primitive(self.assume_nonmut_perform(), Sel::depthAttachmentPixelFormat(), pool, ()))
        }
    }
    pub fn set_depthAttachmentPixelFormat(&mut self, format: MTLPixelFormat, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDepthAttachmentPixelFormat_(), pool, (format.field(),))
        }
    }

}

#[test] fn make_descriptor() {
    autoreleasepool(|pool| {
        let _ = MTLRenderPipelineDescriptor::new(pool);
    })
}
#[test] fn configure_functions() {
    use super::MTLDevice;
    autoreleasepool(|pool| {
        let source = objc_nsstring!("
vertex void vtx(void){}
fragment void frag(void) {}");

        let device = MTLDevice::default().unwrap();
        let _library = device.newLibraryWithSource( &source, None, pool);
        if _library.is_err() {
            println!("{}",_library.as_ref().unwrap_err());
        }
        let mut library = _library.unwrap();
        let vertex_name = objc_nsstring!("vtx");
        let vertex_fn = library.newFunctionWithName(&vertex_name, pool).unwrap();
        let fragment_fn = library.newFunctionWithName( objc_nsstring!("frag"), pool).unwrap();
        let mut descriptor = MTLRenderPipelineDescriptor::new(pool);
        descriptor.set_vertex_function( &vertex_fn,pool);
        assert!(descriptor.description(pool).to_str(pool).contains("name = vtx"));

        descriptor.set_fragment_function( &fragment_fn,pool);
        assert!(descriptor.description(pool).to_str(pool).contains("name = frag"));
    })
}

#[test] fn configure_attachments() {
    autoreleasepool(|pool| {
        let mut descriptor = MTLRenderPipelineDescriptor::new(pool);
        let attachments = descriptor.colorAttachments(pool);
        println!("{}",attachments);

        descriptor.set_depthAttachmentPixelFormat(MTLPixelFormat::R32Sint, pool);
        assert_eq!(descriptor.depthAttachmentPixelFormat(pool), MTLPixelFormat::R32Sint);
    })
}