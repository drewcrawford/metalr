use objr::bindings::*;
objc_instance! {
    pub struct MTLBuffer;
}
objc_selector_group! {
    trait MTLBufferSelectors {
        @selector("contents")
    }
    impl MTLBufferSelectors for Sel {}
}
impl MTLBuffer {
    pub fn contents(&self, pool: &ActiveAutoreleasePool) -> &u8 {
        unsafe {
            let contents: *const u8 = Self::perform_primitive(self.assume_nonmut_perform(), Sel::contents(), pool, ());
            &*contents
        }

    }
    pub fn contents_mut(&mut self, pool: &ActiveAutoreleasePool) -> &mut u8 {
        unsafe {
            let contents: *mut u8 = Self::perform_primitive(self, Sel::contents(), pool, ());
            &mut *contents
        }
    }
}
