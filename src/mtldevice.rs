use objr::bindings::*;
use super::mtlcommandqueue::MTLCommandQueue;
use core::ffi::c_void;
use crate::mtllibrary::MTLLibrary;
use crate::{MTLRenderPipelineDescriptor, MTLTexture, MTLResourceOptions, MTLPixelFormat, MTLRenderPipelineState, MTLSamplerDescriptor,MTLSamplerState};
use std::future::Future;
use blocksr::continuation::Continuation;
use foundationr::{NSInteger, NSUInteger};
use crate::mtlbuffer::MTLBuffer;
use crate::mtldepthstencildescriptor::MTLDepthStencilDescriptor;
use crate::MTLDepthStencilState;
/*
In macOS, in order for the system to provide a default Metal device object, you must link to the CoreGraphics framework.
You usually need to do this explicitly if you are writing apps that don't use graphics by default, such as command line tools.
*/
#[link(name = "CoreGraphics", kind = "framework")]
#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> *mut MTLDevice;
}

objc_enum! {
    pub struct MTLGPUFamily<NSInteger>;
    impl MTLGPUFamily {
        Apple1 = 1001,
    Apple2 = 1002,
    Apple3 = 1003,
   Apple4 = 1004,
    Apple5 = 1005,
    Apple6 = 1006,
    Apple7 = 1007,
        Apple8 = 1008,

    Mac1 = 2001,
    Mac2 = 2002,

    Common1 = 3001,
    Common2 = 3002,
    Common3 = 3003,

    MacCatalyst1 = 4001,
    MacCatalyst2 = 4002
    }
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
        @selector("minimumLinearTextureAlignmentForPixelFormat:")
        @selector("newSamplerStateWithDescriptor:")
        @selector("newDepthStencilStateWithDescriptor:")
        @selector("supportsFamily:")
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
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newTextureWithDescriptor_(), pool, (descriptor.assume_nonmut_perform(),));
            MTLTexture::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn newLibraryWithFile<'a>(&self, file: &NSString, pool: &'a ActiveAutoreleasePool) -> Result<StrongMutCell<MTLLibrary>,AutoreleasedCell<'a, NSError>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform_result(self.assume_nonmut_perform(), Sel::newLibraryWithFile_error(), pool, (file.assume_nonmut_perform(),));
            ptr.map(|d| MTLLibrary::assume_nonnil(d).assume_retained().assume_mut())

        }
    }

    //todo: Implement options
    pub fn newLibraryWithSource<'a>(&self, source: &NSString, _options: Option<()>, pool: &'a ActiveAutoreleasePool)  -> Result<StrongMutCell<MTLLibrary>, AutoreleasedCell<'a, NSError>>{
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform_result(self.assume_nonmut_perform(), Sel::newLibraryWithSource_options_error(), pool, (source.assume_nonmut_perform(), std::ptr::null() as *const c_void));
            ptr.map(|m| MTLLibrary::assume_nonnil(m).assume_retained().assume_mut())
        }
    }

    pub fn newRenderPipelineStateWithDescriptor<'a>(&self, descriptor: &MTLRenderPipelineDescriptor, pool: &'a ActiveAutoreleasePool) -> Result<StrongMutCell<MTLRenderPipelineState>, AutoreleasedCell<'a, NSError>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform_result(self.assume_nonmut_perform(), Sel::newRenderPipelineStateWithDescriptor_error(), pool, (descriptor.assume_nonmut_perform(),));
            ptr.map(|m| MTLRenderPipelineState::assume_nonnil(m).assume_retained().assume_mut())
        }
    }

    fn newRenderPipelineStateWithDescriptorCompletionHandler<F: FnOnce(Result<&mut MTLRenderPipelineState, &NSError>) + Send + 'static>(&self, descriptor: &MTLRenderPipelineDescriptor, pool: &ActiveAutoreleasePool, handler: F)  {
        let block = unsafe{ MTLNewRenderPipelineStateCompletionHandler::new(|pso, error| {
            match pso.as_mut() {
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
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::newRenderPipelineStateWithDescriptor_completionHandler(), pool, (descriptor.assume_nonmut_perform(), &block))
        }
    }

    pub fn newRenderPipelineStateWithDescriptorAsync<'s, 'descriptor,'pool>(&'s self, descriptor: &'descriptor MTLRenderPipelineDescriptor, pool: &'pool ActiveAutoreleasePool) -> impl Future<Output=Result<StrongMutCell<MTLRenderPipelineState>, StrongCell<NSError>>> {
        let (continuation, completion) = Continuation::<(),_>::new();
        //I'm not sure if renderpipelinestate is generally threadsafe, but this usage is public API so it should be fine.
        self.newRenderPipelineStateWithDescriptorCompletionHandler(descriptor, pool, move |result| {
            let result = result.map(|r| StrongMutCell::retaining(r)).map_err(|e| StrongCell::retaining(e));
            let result = unsafe{ImpliedSyncUse::new(result)};
            completion.complete(result);
        });
        async {
            unsafe{continuation.await.unwrap()} //end implied sync use
        }
    }
    pub fn newBufferWithLengthOptions(&self, length: NSUInteger, options: MTLResourceOptions, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLBuffer>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newBufferWithLength_options(), pool, (length,options));
            MTLBuffer::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn minimumLinearTextureAlignmentForPixelFormat(&self, format: MTLPixelFormat, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            //assume_nonmut_perform: see comment above
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::minimumLinearTextureAlignmentForPixelFormat_(), pool,  (format.field(),))
        }
    }
    pub fn newSamplerStateWithDescriptor(&self, descriptor: &MTLSamplerDescriptor, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLSamplerState>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newSamplerStateWithDescriptor_(), pool, (descriptor.assume_nonmut_perform(),));
            MTLSamplerState::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn newDepthStencilStateWithDescriptor(&self, descriptor: &MTLDepthStencilDescriptor, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLDepthStencilState>> {
        unsafe {
            //assume_nonmut_perform: see comment above
            let ptr = Self::perform(self.assume_nonmut_perform(), Sel::newDepthStencilStateWithDescriptor_(), pool, (descriptor.assume_nonmut_perform(),));
            MTLDepthStencilState::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn supportsFamily(&self, family: MTLGPUFamily, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::supportsFamily_(), pool, (family.field(),))
        }
    }


}

#[test] fn test_alignment() {
    let device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let alignment = device.minimumLinearTextureAlignmentForPixelFormat(MTLPixelFormat::R8Unorm, pool);
        println!("alignment,{:?}",alignment);
    });

}

#[test] fn test_descriptor() {
    let device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let descriptor = MTLSamplerDescriptor::new(pool);
        device.newSamplerStateWithDescriptor(&descriptor,pool).unwrap()
    });

}

#[test] fn test_depth_descriptor() {
    let device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let descriptor = MTLDepthStencilDescriptor::new(pool);
        device.newDepthStencilStateWithDescriptor(&descriptor,pool).unwrap()
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

#[test] fn support() {
    let device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        println!("supports apple7 {}",device.supportsFamily(MTLGPUFamily::Apple7, pool));
        println!("supports apple8 {}",device.supportsFamily(MTLGPUFamily::Apple8, pool));
        println!("supports mac1 {}",device.supportsFamily(MTLGPUFamily::Mac1, pool));
        println!("supports mac2 {}",device.supportsFamily(MTLGPUFamily::Mac2, pool));
        println!("supports common1 {}",device.supportsFamily(MTLGPUFamily::Common1, pool));
        println!("supports common2 {}", device.supportsFamily(MTLGPUFamily::Common2, pool));
        println!("supports common3 {}", device.supportsFamily(MTLGPUFamily::Common3, pool));
        println!("supports catalyst1 {}",device.supportsFamily(MTLGPUFamily::MacCatalyst1, pool));
        println!("supports catalyst2 {}",device.supportsFamily(MTLGPUFamily::MacCatalyst2, pool));
        panic!("programmatic failure")
    })
}