use std::ffi::c_void;
use foundationr::NSUInteger;
use objr::bindings::*;
use crate::MTLPixelFormat;
use crate::mtltypes::MTLRegion;
objc_instance! {
    pub struct MTLTexture;
}
//It can be sent to the GPU, so.
unsafe impl Send for MTLTexture {}
unsafe impl Sync for MTLTexture {}

objc_selector_group! {
    trait MTLTextureSelectors {
        @selector("width")
        @selector("height")
        @selector("replaceRegion:mipmapLevel:withBytes:bytesPerRow:")
        @selector("pixelFormat")
    }
    impl MTLTextureSelectors for Sel {}
}
#[allow(non_snake_case)]
impl MTLTexture {
    pub fn width(&self,pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe{Self::perform_primitive(self.assume_nonmut_perform(), Sel::width(), pool, ())}
    }
    pub fn height(&self,pool: &ActiveAutoreleasePool) -> NSUInteger {
        unsafe{Self::perform_primitive(self.assume_nonmut_perform(), Sel::height(), pool, ())}
    }
    pub fn replaceRegion(&mut self, region:MTLRegion, mipmapLevel: NSUInteger, withBytes: *const c_void, bytesPerRow: NSUInteger, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::replaceRegion_mipmapLevel_withBytes_bytesPerRow(), pool, (region, mipmapLevel, withBytes, bytesPerRow))
        }
    }
    pub fn pixelFormat(&self,pool: &ActiveAutoreleasePool) -> MTLPixelFormat {
        MTLPixelFormat(unsafe { Self::perform_primitive(self.assume_nonmut_perform(), Sel::pixelFormat(), pool, ())})
    }
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        use crate::*;

        let device = super::MTLDevice::default().unwrap();
        let texture_descriptor = super::MTLTextureDescriptor::new(pool);
        let texture = device.newTextureWithDescriptor(&texture_descriptor, pool).unwrap();
        assert_eq!(texture.pixelFormat(pool), MTLPixelFormat::RGBA8Unorm);
    })
}
