use objr::bindings::*;
use crate::{MTLBuffer, MTLTexture};
use foundationr::NSUInteger;
use crate::mtltypes::{MTLSize, MTLOrigin};
objc_instance! {
    pub struct MTLBlitCommandEncoder;
}

objc_selector_group! {
    trait MTLBlitCommandEncoderSelectors {
        @selector("copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:")
        @selector("endEncoding")
    }
    impl MTLBlitCommandEncoderSelectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLBlitCommandEncoder {
    pub fn copyFromBuffer(&mut self, buffer: &MTLBuffer, sourceOffset: NSUInteger, sourceBytesPerRow: NSUInteger, sourceBytesPerImage: NSUInteger, sourceSize: MTLSize,
    toTexture: &mut MTLTexture, destinationSlice: NSUInteger, destinationLevel: NSUInteger, destinationOrigin: MTLOrigin, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(),
            pool,
                                    (buffer.assume_nonmut_perform(), sourceOffset, sourceBytesPerRow, sourceBytesPerImage, sourceSize,
                                     toTexture, destinationSlice, destinationLevel, destinationOrigin))
        }

    }
    pub fn endEncoding(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::endEncoding(), pool, ())
        }
    }
}