use objr::bindings::*;
use crate::mtlcommandbuffer::MTLCommandBuffer;

objc_instance! {
    pub struct MTLCommandQueue;
}
objc_selector_group! {
    trait MTLCommandQueueSelectors {
        @selector("commandBuffer")
    }
    impl MTLCommandQueueSelectors for Sel {}
}
impl MTLCommandQueue {
    #[allow(non_snake_case)]
    pub fn commandBuffer(&mut self, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLCommandBuffer>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self, Sel::commandBuffer(), pool, ());
            MTLCommandBuffer::nullable(ptr).assume_retained().assume_mut()
        }
    }
}