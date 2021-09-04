use objr::bindings::*;
use crate::{MTLRenderPipelineState,MTLPrimitiveType};
use foundationr::NSUInteger;

objc_instance! {
    pub struct MTLRenderCommandEncoder;
}
objc_selector_group! {
    trait MTLRenderCommandEncoderSelectors {
        @selector("setRenderPipelineState:")
        @selector("drawPrimitives:vertexStart:vertexCount:")
        @selector("endEncoding")
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
}

#[test] fn smoke_test() {
    use super::*;
    let mut device = MTLDevice::default().unwrap();
    autoreleasepool(|pool| {
        let source = objc_nsstring!("
        vertex float4 vtx() { return float4(1,1,1,1); }
        fragment void frag() { }");
        let mut library = device.newLibraryWithSource(source, None, pool).unwrap();
        let vertex_fn = library.newFunctionWithName( objc_nsstring!("vtx"), pool).unwrap();
        let fragment_fn = library.newFunctionWithName( objc_nsstring!("frag"), pool).unwrap();

        let pass_descriptor = MTLRenderPassDescriptor::new(pool);

        let mut psd = MTLRenderPipelineDescriptor::new(pool);
        psd.set_vertex_function( &vertex_fn,pool);
        psd.set_fragment_function( &fragment_fn,pool);
        let result = device.newRenderPipelineStateWithDescriptor( &psd, pool);
        let pso = result.unwrap();
        let mut queue = device.newCommandQueue(pool).unwrap();
        let mut buffer = queue.commandBuffer(pool).unwrap();
        let mut encoder = buffer.renderCommandEncoderWithDescriptor( &pass_descriptor, pool).unwrap();
        encoder.setRenderPipelineState( &pso, pool);
        encoder.drawPrimitivesVertexStartVertexCount( MTLPrimitiveType::Triangle,0,3, pool);
        encoder.endEncoding(pool);
    })
}