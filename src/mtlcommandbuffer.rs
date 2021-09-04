use objr::bindings::*;
use super::{MTLRenderPassDescriptor,MTLRenderCommandEncoder,MTLDrawable};
objc_instance!(
    pub struct MTLCommandBuffer;
);

objc_selector_group! {
    trait MTLCommandBufferSelectors {
        @selector("renderCommandEncoderWithDescriptor:")
        @selector("commit")
        @selector("presentDrawable:")
    }
    impl MTLCommandBufferSelectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLCommandBuffer {
    pub fn renderCommandEncoderWithDescriptor(&mut self, pool: &ActiveAutoreleasePool, descriptor: &MTLRenderPassDescriptor) -> Option<StrongMutCell<MTLRenderCommandEncoder>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::renderCommandEncoderWithDescriptor_(), pool, (descriptor,));
            MTLRenderCommandEncoder::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn commit(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe{ Self::perform_primitive(self, Sel::commit(), pool, ()) }
    }
    pub fn presentDrawable(&mut self, pool: &ActiveAutoreleasePool, drawable: &MTLDrawable) {
        unsafe{ Self::perform_primitive(self,Sel::presentDrawable_(), pool, (drawable,)) }
    }
}

#[test] fn smoke_test() {
    use super::*;
    autoreleasepool(|pool| {
        let mut device = MTLDevice::default().unwrap();
        let mut command_q = device.newCommandQueue(pool).unwrap();
        let mut command_buffer = command_q.commandBuffer(pool).unwrap();
        let descriptor = MTLRenderPassDescriptor::new(pool);
        let mut render_pass = command_buffer.renderCommandEncoderWithDescriptor(pool, &descriptor).unwrap();
        render_pass.endEncoding(pool);
        command_buffer.commit(pool);
    });

}