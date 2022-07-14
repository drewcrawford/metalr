use foundationr::{NSUInteger,NSRange};
use objr::bindings::*;
use crate::{MTLTextureDescriptor, MTLTexture};

objc_instance! {
    pub struct MTLBuffer;
}
objc_selector_group! {
    trait MTLBufferSelectors {
        @selector("contents")
        @selector("newTextureWithDescriptor:offset:bytesPerRow:")
        @selector("length")
        @selector("didModifyRange:")
        @selector("addDebugMarker:range:")
    }
    impl MTLBufferSelectors for Sel {}
}
//we wrap all contents accesses in `unsafe`
unsafe impl Send for MTLBuffer {}
unsafe impl Sync for MTLBuffer {}
#[allow(non_snake_case)]
impl MTLBuffer {
    ///
    /// # Safety
    /// You must ensure your access does not overlap any concurrent accesses.
    pub unsafe fn contents(&self, pool: &ActiveAutoreleasePool) -> &u8 {
        let contents: *const u8 = Self::perform_primitive(self.assume_nonmut_perform(), Sel::contents(), pool, ());
        &*contents
    }
    pub fn length(&self, pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::length(), pool, ())
        }
    }
    ///
    /// # Safety
    /// You must ensure your access does not overlap any concurrent accesses.
    pub fn contents_mut(&mut self, pool: &ActiveAutoreleasePool) -> &mut u8 {
        unsafe {
            let contents: *const u8 = Self::perform_primitive(self, Sel::contents(), pool, ());
            //safe because we are mut ourselves
            &mut* (contents as *mut u8)
        }
    }
    pub fn newTextureWithDescriptor(&mut self, descriptor: &MTLTextureDescriptor, offset: NSUInteger, bytesPerRow: NSUInteger, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLTexture>> {
        unsafe {
            let raw = Self::perform(self, Sel::newTextureWithDescriptor_offset_bytesPerRow(), pool, (descriptor.assume_nonmut_perform(), offset, bytesPerRow));
            MTLTexture::nullable(raw).assume_retained().assume_mut()
        }
    }
    pub fn didModifyRange(&mut self, range: NSRange, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::didModifyRange_(), pool, (range,))
        }
    }
    pub fn addDebugMarker(&mut self, marker: &NSString, range: NSRange, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::addDebugMarker_range(), pool, (marker.assume_nonmut_perform(),range))
        }
    }
}

#[test] fn smoke() {
    use crate::*;
    let device = crate::MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let mut buffer = device.newBufferWithLengthOptions(1, MTLResourceOptions::with_options(MTLCPUCacheMode::DefaultCache, MTLStorageMode::Private, MTLHazardTrackingMode::Default),pool).unwrap();
        buffer.addDebugMarker(objc_nsstring!("smoke_test"), NSRange{ location: 0, length: 0 }, pool)
    })
}

