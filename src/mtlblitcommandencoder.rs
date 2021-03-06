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
        @selector("optimizeContentsForGPUAccess:")
        @selector("copyFromTexture:toTexture:")
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
    pub fn copyFromTextureToTexture(&mut self, fromTexture: &MTLTexture, toTexture: &mut MTLTexture, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::copyFromTexture_toTexture(), pool, (fromTexture.assume_nonmut_perform(), toTexture))
        }
    }

    pub fn optimizeContentsForGPUAccess(&mut self, texture: &mut MTLTexture, pool: &ActiveAutoreleasePool) {
        unsafe{Self::perform_primitive(self, Sel::optimizeContentsForGPUAccess_(), pool, (texture,))}
    }
    pub fn endEncoding(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::endEncoding(), pool, ())
        }
    }
}

#[test] fn smoke() {
    use crate::*;
    autoreleasepool(|pool| {
        let device = MTLDevice::default().unwrap();
        let command_q = device.newCommandQueue(pool).unwrap();
        let mut command_buffer = command_q.commandBuffer(pool).unwrap();
        let mut encoder = command_buffer.blitCommandEncoder(pool).unwrap();
        let texture_descriptor = MTLTextureDescriptor::new(pool);
        let mut texture = device.newTextureWithDescriptor(&texture_descriptor,pool).unwrap();
        let texture2 = device.newTextureWithDescriptor(&texture_descriptor, pool).unwrap();
        encoder.copyFromTextureToTexture(&texture2,&mut texture,pool);
        encoder.optimizeContentsForGPUAccess(&mut texture, pool);
        encoder.endEncoding(pool);
    })

}