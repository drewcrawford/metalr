use objr::bindings::*;
use super::mtlcommandqueue::MTLCommandQueue;
use core::ffi::c_void;
use crate::mtllibrary::MTLLibrary;
use crate::{MTLRenderPipelineDescriptor, MTLTexture, MTLResourceOptions};
use crate::MTLRenderPipelineState;
use std::future::Future;
use blocksr::continuation::Continuation;
use foundationr::NSUInteger;
use crate::mtlbuffer::MTLBuffer;
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
        @selector("newBufferWithLength:options:")
        @selector("newSamplerStateWithDescriptor:")
    }
    impl MTLDeviceSelectors for Sel {}
}

blocksr::once_escaping!(MTLNewRenderPipelineStateCompletionHandler (render_pipeline_state: *mut MTLRenderPipelineState, error: *const NSError) -> ());
unsafe impl Arguable for &MTLNewRenderPipelineStateCompletionHandler {}

/*
A note on threadsafety (e.g., mutability) in here.

MTLDevice is both allegedly threadsafe (see gfx engineer comment here: https://developer.apple.com/forums/thread/93346)
and known not to be (see rust-gfs https://github.com/gfx-rs/gfx/issues/1976 and https://github.com/gfx-rs/gfx/issues/1984).

rust-gfx solves this with runtime locks.  I'm not sure if this is needed or not.  For right now, let's just use assume_nonmut on
functions in this file.
 */
#[allow(non_snake_case)]
impl MTLDevice {
    pub fn default() -> Option<StrongMutCell<Self>> {
        unsafe {
            let raw_ptr = MTLCreateSystemDefaultDevice();
            Self::nullable(raw_ptr).assume_retained().assume_mut()
        }
    }
    pub fn newCommandQueue(&self, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLCommandQueue>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newCommandQueue(), pool, ());
            MTLCommandQueue::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn newTextureWithDescriptor(&self,  descriptor: &super::MTLTextureDescriptor, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLTexture>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newTextureWithDescriptor_(), pool, (descriptor,));
            MTLTexture::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn newLibraryWithFile<'a>(&self, file: &NSString, pool: &'a ActiveAutoreleasePool) -> Result<StrongMutCell<MTLLibrary>,AutoreleasedCell<'a, NSError>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform_result(self.assume_nonmut_perform(), Sel::newLibraryWithFile_error(), pool, (file,));
            ptr.map(|d| MTLLibrary::assume_nonnil(d).assume_retained().assume_mut())

        }
    }

    //todo: Implement options
    pub fn newLibraryWithSource<'a>(&self, source: &NSString, _options: Option<()>, pool: &'a ActiveAutoreleasePool)  -> Result<StrongMutCell<MTLLibrary>, AutoreleasedCell<'a, NSError>>{
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform_result(self.assume_nonmut_perform(), Sel::newLibraryWithSource_options_error(), pool, (source, std::ptr::null() as *const c_void));
            ptr.map(|m| MTLLibrary::assume_nonnil(m).assume_retained().assume_mut())
        }
    }

    pub fn newRenderPipelineStateWithDescriptor<'a>(&self, descriptor: &MTLRenderPipelineDescriptor, pool: &'a ActiveAutoreleasePool) -> Result<StrongCell<MTLRenderPipelineState>, AutoreleasedCell<'a, NSError>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform_result(self.assume_nonmut_perform(), Sel::newRenderPipelineStateWithDescriptor_error(), pool, (descriptor,));
            ptr.map(|m| MTLRenderPipelineState::assume_nonnil(m).assume_retained())
        }
    }

    fn newRenderPipelineStateWithDescriptorCompletionHandler<F: FnOnce(Result<&MTLRenderPipelineState, &NSError>) + Send + 'static>(&self, descriptor: &MTLRenderPipelineDescriptor, pool: &ActiveAutoreleasePool, handler: F)  {
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
            //assume_nonmut_perform: see comment above
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::newRenderPipelineStateWithDescriptor_completionHandler(), pool, (descriptor, &block))
        }
    }

    pub fn newRenderPipelineStateWithDescriptorAsync<'s, 'descriptor,'pool>(&'s self, descriptor: &'descriptor MTLRenderPipelineDescriptor, pool: &'pool ActiveAutoreleasePool) -> impl Future<Output=Result<StrongCell<MTLRenderPipelineState>, StrongCell<NSError>>> {
        let (continuation, completion) = Continuation::<(),_>::new();
        self.newRenderPipelineStateWithDescriptorCompletionHandler(descriptor, pool, |result| {
            let result = result.map(|r| StrongCell::retaining(r)).map_err(|e| StrongCell::retaining(e));
            completion.complete(result);
        });
        continuation
    }
    pub fn newBufferWithLengthOptions(&self, length: NSUInteger, options: MTLResourceOptions, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLBuffer>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newBufferWithLength_options(), pool, (length,options));
            MTLBuffer::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn newSamplerStateWithDescriptor(&self, descriptor: &MTLSamplerDescriptor, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLSamplerState>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newSamplerStateWithDescriptor_(), pool, (descriptor,));
            MTLSamplerState::nullable(ptr).assume_retained().assume_mut()
        }
    }


}

#[test] fn test_descriptor() {
    let device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let descriptor = MTLSamplerDescriptor::new(pool);
        device.newSamplerStateWithDescriptor(&descriptor,pool).unwrap()
    });
}

#[test] fn test_source() {
    let device = MTLDevice::default().unwrap();
    let source = objc_nsstring!("kernel void func() { }");

    autoreleasepool(|pool| {
        let result = device.newLibraryWithSource( &source, None, pool);
        result.expect("Expected a library");
    })
}

#[test] fn pso() {
    let device = MTLDevice::default().unwrap();
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
        unsafe{psd.colorAttachments(pool).objectAtIndexedSubscript(0, pool)}.set_pixelFormat(crate::MTLPixelFormat::BGRA8Unorm,pool);
        let result = device.newRenderPipelineStateWithDescriptor( &psd, pool);
        let e = result.unwrap();
        println!("{}",e);

    })
}