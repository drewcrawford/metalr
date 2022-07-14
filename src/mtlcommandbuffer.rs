use std::future::Future;
use blocksr::continuation::Continuation;
use objr::bindings::*;
use super::{MTLRenderPassDescriptor,MTLRenderCommandEncoder,MTLDrawable};
use crate::mtlblitcommandencoder::MTLBlitCommandEncoder;
objc_instance!(
    pub struct MTLCommandBuffer;
);

objc_selector_group! {
    trait MTLCommandBufferSelectors {
        @selector("renderCommandEncoderWithDescriptor:")
        @selector("commit")
        @selector("presentDrawable:")
        @selector("blitCommandEncoder")
        @selector("addCompletedHandler:")
    }
    impl MTLCommandBufferSelectors for Sel {}
}
blocksr::once_escaping!(MTLCommandBufferHandler(buffer: &MTLCommandBuffer) -> ());
unsafe impl Arguable for &MTLCommandBufferHandler {}

#[allow(non_snake_case)]
impl MTLCommandBuffer {
    pub fn renderCommandEncoderWithDescriptor(&mut self,  descriptor: &MTLRenderPassDescriptor, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLRenderCommandEncoder>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::renderCommandEncoderWithDescriptor_(), pool, (descriptor.assume_nonmut_perform(),));
            MTLRenderCommandEncoder::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn commit(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe{ Self::perform_primitive(self, Sel::commit(), pool, ()) }
    }
    pub fn presentDrawable(&mut self, pool: &ActiveAutoreleasePool, drawable: &MTLDrawable) {
        unsafe{ Self::perform_primitive(self,Sel::presentDrawable_(), pool, (drawable.assume_nonmut_perform(),)) }
    }
    pub fn blitCommandEncoder(&mut self, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLBlitCommandEncoder>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::blitCommandEncoder(), pool, ());
            MTLBlitCommandEncoder::nullable(ptr).assume_retained().assume_mut()
        }
    }
    pub fn addCompletedHandler<C: FnOnce(&MTLCommandBuffer) + Send + 'static>(&mut self, handler: C, pool: &ActiveAutoreleasePool) {
        unsafe {
            let block = MTLCommandBufferHandler::new(handler);
            Self::perform_primitive(self, Sel::addCompletedHandler_(), pool, (&block,))
        }
    }

    /**
    A device that
    1.  Adds a completion handler
    2.  commits
    3.  awaits the completion handler
    */
    pub fn commitWaitUntilCompletedAsync(&mut self, pool: &ActiveAutoreleasePool) -> impl Future<Output=()> {
        let (continuation,completer) = Continuation::<(),_>::new();
        self.addCompletedHandler(|_| {
            completer.complete(())
        },pool);
        self.commit(pool);
        continuation
    }

}

#[test] fn smoke_test() {
    use super::*;
    autoreleasepool(|pool| {
        let device = MTLDevice::default().unwrap();
        let command_q = device.newCommandQueue(pool).unwrap();
        let mut command_buffer = command_q.commandBuffer(pool).unwrap();
        let mut descriptor = MTLRenderPassDescriptor::new(pool);
        descriptor.set_renderTargetHeight(100, pool);
        descriptor.set_renderTargetWidth(100, pool);
        let mut render_pass = command_buffer.renderCommandEncoderWithDescriptor(&descriptor, pool).unwrap();
        render_pass.endEncoding(pool);
        let (sender,receiver) = std::sync::mpsc::sync_channel(1);
        command_buffer.addCompletedHandler(move |_| {
            sender.send(()).unwrap();
        },pool);
        command_buffer.commit(pool);
        receiver.recv().unwrap();

    });

}