use std::ffi::c_void;
use foundationr::NSUInteger;
use objr::bindings::*;
use crate::mtltypes::MTLRegion;
objc_instance! {
    pub struct MTLTexture;
}

objc_selector_group! {
    trait MTLTextureSelectors {
        @selector("width")
        @selector("height")
        @selector("replaceRegion:mipmapLevel:withBytes:bytesPerRow:")
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
}

