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
    pub fn setRenderPipelineState(&mut self, pool: &ActiveAutoreleasePool, pipelineState: &MTLRenderPipelineState) {
        unsafe {
            Self::perform_primitive(self, Sel::setRenderPipelineState_(), pool, (pipelineState,))
        }
    }
    pub fn drawPrimitivesVertexStartVertexCount(&mut self, pool: &ActiveAutoreleasePool, primitive: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger) {
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
        let mut library = device.newLibraryWithSource(pool, source, None).unwrap();
        let vertex_fn = library.newFunctionWithName(pool, objc_nsstring!("vtx")).unwrap();
        let fragment_fn = library.newFunctionWithName(pool, objc_nsstring!("frag")).unwrap();

        let pass_descriptor = MTLRenderPassDescriptor::new(pool);

        let mut psd = MTLRenderPipelineDescriptor::new(pool);
        psd.set_vertex_function(pool, &vertex_fn);
        psd.set_fragment_function(pool, &fragment_fn);
        let result = device.newRenderPipelineStateWithDescriptor(pool, &psd);
        let pso = result.unwrap();
        let mut queue = device.newCommandQueue(pool).unwrap();
        let mut buffer = queue.commandBuffer(pool).unwrap();
        let mut encoder = buffer.renderCommandEncoderWithDescriptor(pool, &pass_descriptor).unwrap();
        encoder.setRenderPipelineState(pool, &pso);
        encoder.drawPrimitivesVertexStartVertexCount(pool, MTLPrimitiveType::Triangle,0,3);
        encoder.endEncoding(pool);
    })
}