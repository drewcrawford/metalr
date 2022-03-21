use std::ffi::c_void;
use objr::bindings::*;
use crate::{MTLRenderPipelineState, MTLPrimitiveType, MTLTexture,MTLSamplerState,MTLDepthStencilState};
use foundationr::NSUInteger;

objc_instance! {
    pub struct MTLRenderCommandEncoder;
}
objc_selector_group! {
    trait MTLRenderCommandEncoderSelectors {
        @selector("setRenderPipelineState:")
        @selector("drawPrimitives:vertexStart:vertexCount:")
        @selector("endEncoding")
        @selector("setFragmentTexture:atIndex:")
        @selector("setFragmentSamplerState:atIndex:")
        @selector("setVertexTexture:atIndex:")
        @selector("setDepthStencilState:")
        @selector("setVertexBytes:length:atIndex:")
    }
    impl MTLRenderCommandEncoderSelectors for Sel {}
}
#[allow(non_snake_case)]
impl MTLRenderCommandEncoder {
    pub fn setRenderPipelineState(&mut self, pipelineState: &MTLRenderPipelineState, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setRenderPipelineState_(), pool, (pipelineState,))
        }
    }
    pub fn setFragmentTextureAtIndex(&mut self, texture: Option<&MTLTexture>, index: NSUInteger, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setFragmentTexture_atIndex(), pool, (texture.as_ptr(),index))
        }
    }
    pub fn setVertexTextureAtIndex(&mut self, texture: Option<&MTLTexture>, index: NSUInteger, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setVertexTexture_atIndex(), pool, (texture.as_ptr(),index))
        }
    }
    ///# Safety
    /// Length is unverified.
    ///
    /// For a safe wrapper, consider [setVertexBytesFromType].
    pub unsafe fn setVertexBytesLengthAtIndex(&mut self, bytes: *const c_void, length: NSUInteger, atIndex: NSUInteger, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self,Sel::setVertexBytes_length_atIndex(),pool, (bytes,length,atIndex))
    }
    pub fn setVertexBytesFromType<T>(&mut self, r#type: T, atIndex: NSUInteger, pool: &ActiveAutoreleasePool) {
        unsafe {
            self.setVertexBytesLengthAtIndex(&r#type as *const T as *const c_void,std::mem::size_of::<T>() as u64, atIndex,pool)
        }
    }
    pub fn setFragmentSamplerStateAtIndex(&mut self, sampler: &MTLSamplerState, index: NSUInteger, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::setFragmentSamplerState_atIndex(), pool, (sampler,index))
        }
    }

    pub fn drawPrimitivesVertexStartVertexCount(&mut self, primitive: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger,pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::drawPrimitives_vertexStart_vertexCount(), pool, (primitive.field(), vertexStart, vertexCount))
        }
    }
    pub fn endEncoding(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::endEncoding(), pool, ())
        }
    }
    pub fn setDepthStencilState(&mut self, state: Option<&MTLDepthStencilState>, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDepthStencilState_(),pool,(state.as_ptr(),))
        }
    }
}

#[test] fn smoke_test() {
    use super::*;
    let device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let source = objc_nsstring!("
        vertex float4 vtx() { return float4(1,1,1,1); }
        fragment void frag() { }");
        let mut library = device.newLibraryWithSource(source, None, pool).unwrap();
        let vertex_fn = library.newFunctionWithName( objc_nsstring!("vtx"), pool).unwrap();
        let fragment_fn = library.newFunctionWithName( objc_nsstring!("frag"), pool).unwrap();

        let mut pass_descriptor = MTLRenderPassDescriptor::new(pool);
        //on apple silicon, we need a real size
        pass_descriptor.set_renderTargetHeight(100, pool);
        pass_descriptor.set_renderTargetWidth(100, pool);
        let mut psd = MTLRenderPipelineDescriptor::new(pool);
        psd.set_vertex_function( &vertex_fn,pool);
        psd.set_fragment_function( &fragment_fn,pool);
        unsafe{psd.colorAttachments(pool).objectAtIndexedSubscript(0, pool)}.set_pixelFormat(crate::MTLPixelFormat::BGRA8Unorm,pool);
        let result = device.newRenderPipelineStateWithDescriptor( &psd, pool);
        let pso = result.unwrap();
        let queue = device.newCommandQueue(pool).unwrap();
        let mut buffer = queue.commandBuffer(pool).unwrap();
        let mut encoder = buffer.renderCommandEncoderWithDescriptor( &pass_descriptor, pool).unwrap();
        encoder.setRenderPipelineState( &pso, pool);
        encoder.setVertexBytesFromType(2 as u32, 0,pool);
        encoder.drawPrimitivesVertexStartVertexCount( MTLPrimitiveType::Triangle,0,3, pool);
        encoder.endEncoding(pool);
    })
}