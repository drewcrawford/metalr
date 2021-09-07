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
//allegedly
unsafe impl Sync for MTLCommandQueue {}
impl MTLCommandQueue {
    #[allow(non_snake_case)]
    //marking this nonmut because the class is documented to be threadsafe
    pub fn commandBuffer(&self, pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<MTLCommandBuffer>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::commandBuffer(), pool, ());
            MTLCommandBuffer::nullable(ptr).assume_retained().assume_mut()
        }
    }
}