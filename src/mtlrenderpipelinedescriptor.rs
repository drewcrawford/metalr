use objr::bindings::*;
use crate::{MTLFunction, MTLRenderPipelineColorAttachmentDescriptorArray};
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
    }
    impl MTLRenderPipelineDescriptorSelectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLRenderPipelineDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe{ MTLRenderPipelineDescriptor::class().alloc_init(pool).assume_mut() }
    }
    pub fn set_vertex_function(&mut self, pool: &ActiveAutoreleasePool, function: &MTLFunction) {
        unsafe {
            Self::perform_primitive(self, Sel::setVertexFunction_(), pool, (function,))
        }
    }
    pub fn set_fragment_function(&mut self, pool: &ActiveAutoreleasePool, function: &MTLFunction) {
        unsafe {
            Self::perform_primitive(self, Sel::setFragmentFunction_(), pool, (function,))
        }
    }
    pub fn colorAttachments(&mut self, pool: &ActiveAutoreleasePool) -> StrongCell<MTLRenderPipelineColorAttachmentDescriptorArray> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::colorAttachments(), pool, ());
            MTLRenderPipelineColorAttachmentDescriptorArray::assume_nonnil(ptr).assume_retained()
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

        let mut device = MTLDevice::default().unwrap();
        let _library = device.newLibraryWithSource(pool, &source, None);
        if _library.is_err() {
            println!("{}",_library.as_ref().unwrap_err());
        }
        let mut library = _library.unwrap();
        let vertex_name = objc_nsstring!("vtx");
        let vertex_fn = library.newFunctionWithName(pool,&vertex_name).unwrap();
        let fragment_fn = library.newFunctionWithName(pool, objc_nsstring!("frag")).unwrap();
        let mut descriptor = MTLRenderPipelineDescriptor::new(pool);
        descriptor.set_vertex_function(pool, &vertex_fn);
        assert!(descriptor.description(pool).to_str(pool).contains("name = vtx"));

        descriptor.set_fragment_function(pool, &fragment_fn);
        assert!(descriptor.description(pool).to_str(pool).contains("name = frag"));
    })
}

#[test] fn configure_attachments() {
    autoreleasepool(|pool| {
        let mut descriptor = MTLRenderPipelineDescriptor::new(pool);
        let attachments = descriptor.colorAttachments(pool);
        println!("{}",attachments);
    })
}