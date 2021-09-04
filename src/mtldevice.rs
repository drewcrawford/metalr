use objr::bindings::*;
use super::mtlcommandqueue::MTLCommandQueue;
use core::ffi::c_void;
use crate::mtllibrary::MTLLibrary;
use crate::{MTLRenderPipelineDescriptor, MTLTexture};
use crate::MTLRenderPipelineState;
/*
In macOS, in order for the system to provide a default Metal device object, you must link to the CoreGraphics framework.
You usually need to do this explicitly if you are writing apps that don't use graphics by default, such as command line tools.
*/
#[link(name = "CoreGraphics", kind = "framework")]
#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> *mut MTLDevice;
}
objc_instance! {
    pub struct MTLDevice;
}
objc_selector_group! {
    trait MTLDeviceSelectors {
        @selector("newCommandQueue")
        @selector("newTextureWithDescriptor:")
        @selector("newLibraryWithFile:error:")
        @selector("newLibraryWithSource:options:error:")
        @selector("newRenderPipelineStateWithDescriptor:error:")
    }
    impl MTLDeviceSelectors for Sel {}
}
#[allow(non_snake_case)]
impl MTLDevice {
    pub fn default() -> Option<StrongMutCell<Self>> {
        unsafe {
            let raw_ptr = MTLCreateSystemDefaultDevice();
            Self::nullable(raw_ptr).assume_retained().assume_mut()
        }
    }
    pub fn newCommandQueue(&mut self, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLCommandQueue>> {
        unsafe {
            let ptr = Self::perform(self, Sel::newCommandQueue(), pool, ());
            MTLCommandQueue::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn newTextureWithDescriptor(&mut self, pool: &ActiveAutoreleasePool, descriptor: &super::MTLTextureDescriptor) -> Option<StrongCell<MTLTexture>> {
        unsafe {
            let ptr = Self::perform(self, Sel::newTextureWithDescriptor_(), pool, (descriptor,));
            MTLTexture::nullable(ptr).assume_retained()
        }
    }
    pub fn newLibraryWithFile<'a>(&mut self, pool: &'a ActiveAutoreleasePool, file: &NSString) -> Result<StrongCell<MTLLibrary>,AutoreleasedCell<'a, NSError>> {
        unsafe {
            let ptr = Self::perform_result(self, Sel::newLibraryWithFile_error(), pool, (file,));
            ptr.map(|d| MTLLibrary::assume_nonnil(d).assume_retained())

        }
    }

    //todo: Implement options
    pub fn newLibraryWithSource<'a>(&mut self, pool: &'a ActiveAutoreleasePool, source: &NSString, _options: Option<()>)  -> Result<StrongMutCell<MTLLibrary>, AutoreleasedCell<'a, NSError>>{
        unsafe {
            let ptr = Self::perform_result(self, Sel::newLibraryWithSource_options_error(), pool, (source, std::ptr::null() as *const c_void));
            ptr.map(|m| MTLLibrary::assume_nonnil(m).assume_retained().assume_mut())
        }
    }

    pub fn newRenderPipelineStateWithDescriptor<'a>(&mut self, pool: &'a ActiveAutoreleasePool, descriptor: &MTLRenderPipelineDescriptor) -> Result<StrongCell<MTLRenderPipelineState>, AutoreleasedCell<'a, NSError>> {
        unsafe {
            let ptr = Self::perform_result(self, Sel::newRenderPipelineStateWithDescriptor_error(), pool, (descriptor,));
            ptr.map(|m| MTLRenderPipelineState::assume_nonnil(m).assume_retained())
        }
    }

}

#[test] fn test_source() {
    let mut device = MTLDevice::default().unwrap();
    let source = objc_nsstring!("kernel void func() { }");

    autoreleasepool(|pool| {
        let result = device.newLibraryWithSource(pool, &source, None);
        result.expect("Expected a library");
    })
}

#[test] fn pso() {
    let mut device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let source = objc_nsstring!("
        vertex float4 vtx() { return float4(1,1,1,1); }
        fragment void frag() { }");
        let mut library = device.newLibraryWithSource(pool, source, None).unwrap();
        let vertex_fn = library.newFunctionWithName(pool, objc_nsstring!("vtx")).unwrap();
        let fragment_fn = library.newFunctionWithName(pool, objc_nsstring!("frag")).unwrap();

        let mut psd = MTLRenderPipelineDescriptor::new(pool);
        psd.set_vertex_function(pool, &vertex_fn);
        psd.set_fragment_function(pool, &fragment_fn);
        let result = device.newRenderPipelineStateWithDescriptor(pool, &psd);
        let e = result.unwrap();
        println!("{}",e);

    })
}