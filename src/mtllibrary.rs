use objr::bindings::*;
use super::MTLFunction;
objc_instance! {
    pub struct MTLLibrary;
}
objc_selector_group! {
    trait MTLLibrarySelectors {
        @selector("newFunctionWithName:")
    }
    impl MTLLibrarySelectors for Sel{}
}

#[allow(non_snake_case)]
impl MTLLibrary {
    pub fn newFunctionWithName(&mut self, name: &NSString, pool: &ActiveAutoreleasePool) -> Option<StrongCell<MTLFunction>> {
        unsafe {
            let ptr = Self::perform(self, Sel::newFunctionWithName_(), pool, (name,));
            MTLFunction::nullable(ptr).assume_retained()
        }
    }
}

