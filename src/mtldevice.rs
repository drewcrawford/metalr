use objr::bindings::*;
use super::mtlcommandqueue::MTLCommandQueue;
use core::ffi::c_void;
use crate::mtllibrary::MTLLibrary;
use crate::{MTLRenderPipelineDescriptor, MTLTexture};
use crate::MTLRenderPipelineState;
use std::future::Future;
use blocksr::continuation::Continuation;
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
        @selector("newRenderPipelineStateWithDescriptor:completionHandler:")
    }
    impl MTLDeviceSelectors for Sel {}
}

blocksr::once_escaping!(MTLNewRenderPipelineStateCompletionHandler (render_pipeline_state: *mut MTLRenderPipelineState, error: *const NSError) -> ());
unsafe impl Arguable for &MTLNewRenderPipelineStateCompletionHandler {}

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
    pub fn newTextureWithDescriptor(&mut self,  descriptor: &super::MTLTextureDescriptor, pool: &ActiveAutoreleasePool) -> Option<StrongCell<MTLTexture>> {
        unsafe {
            let ptr = Self::perform(self, Sel::newTextureWithDescriptor_(), pool, (descriptor,));
            MTLTexture::nullable(ptr).assume_retained()
        }
    }
    pub fn newLibraryWithFile<'a>(&mut self, file: &NSString, pool: &'a ActiveAutoreleasePool) -> Result<StrongMutCell<MTLLibrary>,AutoreleasedCell<'a, NSError>> {
        unsafe {
            let ptr = Self::perform_result(self, Sel::newLibraryWithFile_error(), pool, (file,));
            ptr.map(|d| MTLLibrary::assume_nonnil(d).assume_retained().assume_mut())

        }
    }

    //todo: Implement options
    pub fn newLibraryWithSource<'a>(&mut self, source: &NSString, _options: Option<()>, pool: &'a ActiveAutoreleasePool)  -> Result<StrongMutCell<MTLLibrary>, AutoreleasedCell<'a, NSError>>{
        unsafe {
            let ptr = Self::perform_result(self, Sel::newLibraryWithSource_options_error(), pool, (source, std::ptr::null() as *const c_void));
            ptr.map(|m| MTLLibrary::assume_nonnil(m).assume_retained().assume_mut())
        }
    }

    pub fn newRenderPipelineStateWithDescriptor<'a>(&mut self, descriptor: &MTLRenderPipelineDescriptor, pool: &'a ActiveAutoreleasePool) -> Result<StrongCell<MTLRenderPipelineState>, AutoreleasedCell<'a, NSError>> {
        unsafe {
            let ptr = Self::perform_result(self, Sel::newRenderPipelineStateWithDescriptor_error(), pool, (descriptor,));
            ptr.map(|m| MTLRenderPipelineState::assume_nonnil(m).assume_retained())
        }
    }

    fn newRenderPipelineStateWithDescriptorCompletionHandler<F: FnOnce(Result<&MTLRenderPipelineState, &NSError>) + Send + 'static>(&mut self, descriptor: &MTLRenderPipelineDescriptor, pool: &ActiveAutoreleasePool, handler: F)  {
        let block = unsafe{ MTLNewRenderPipelineStateCompletionHandler::new(|pso, error| {
            match pso.as_ref() {
               None => {
                   handler(Err(error.as_ref().unwrap()))
               }
               Some(pso) => {
                   handler(Ok(pso))
               }
           }
        })};
        unsafe {
            Self::perform_primitive(self, Sel::newRenderPipelineStateWithDescriptor_completionHandler(), pool, (descriptor, &block))
        }
    }

    pub fn newRenderPipelineStateWithDescriptorAsync<'s, 'descriptor,'pool>(&'s mut self, descriptor: &'descriptor MTLRenderPipelineDescriptor, pool: &'pool ActiveAutoreleasePool) -> impl Future<Output=Result<StrongCell<MTLRenderPipelineState>, StrongCell<NSError>>> {
        let (continuation, completion) = Continuation::<(),_>::new();
        self.newRenderPipelineStateWithDescriptorCompletionHandler(descriptor, pool, |result| {
            let result = result.map(|r| StrongCell::retaining(r)).map_err(|e| StrongCell::retaining(e));
            completion.complete(result);
        });
        continuation
    }


}

#[test] fn test_source() {
    let mut device = MTLDevice::default().unwrap();
    let source = objc_nsstring!("kernel void func() { }");

    autoreleasepool(|pool| {
        let result = device.newLibraryWithSource( &source, None, pool);
        result.expect("Expected a library");
    })
}

#[test] fn pso() {
    let mut device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let source = objc_nsstring!("
        vertex float4 vtx() { return float4(1,1,1,1); }
        fragment void frag() { }");
        let mut library = device.newLibraryWithSource( source, None, pool).unwrap();
        let vertex_fn = library.newFunctionWithName( objc_nsstring!("vtx"), pool).unwrap();
        let fragment_fn = library.newFunctionWithName(objc_nsstring!("frag"), pool).unwrap();

        let mut psd = MTLRenderPipelineDescriptor::new(pool);
        psd.set_vertex_function( &vertex_fn,pool);
        psd.set_fragment_function( &fragment_fn,pool);
        let result = device.newRenderPipelineStateWithDescriptor( &psd, pool);
        let e = result.unwrap();
        println!("{}",e);

    })
}