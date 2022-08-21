use foundationr::{NSUInteger,NSRange};
use objr::bindings::*;
use crate::{MTLTextureDescriptor, MTLTexture,MTLResource};

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

impl PartialEq for MTLBuffer {
    fn eq(&self, other: &Self) -> bool {
        (self as *const MTLBuffer) == (other as *const MTLBuffer)
    }
}
impl Eq for MTLBuffer {}
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
    pub const fn as_resource(&self) -> &MTLResource {
        unsafe { &* (self as *const _ as *const MTLResource) }
    }
    pub fn as_resource_mut(&mut self) -> &mut MTLResource {
        unsafe { &mut * (self as *mut _ as *mut MTLResource) }
    }
}
impl<'a> From<&'a MTLBuffer> for &'a MTLResource {
    fn from(t: &'a MTLBuffer) -> Self {
        t.as_resource()
    }
}
impl<'a> From<&'a mut MTLBuffer> for &'a mut MTLResource {
    fn from(t: &'a mut MTLBuffer) -> Self {
        t.as_resource_mut()
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

