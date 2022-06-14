use objr::bindings::*;
use crate::mtlargument::MTLDataType;
objc_class! {
    pub struct MTLFunctionConstantValues {
        @class(MTLFunctionConstantValues)
    }
}
objc_selector_group! {
    trait Sel {
        @selector("setConstantValue:type:atIndex:")
    }
    impl Sel for Sel {}
}

#[allow(non_snake_case)]
impl MTLFunctionConstantValues {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<MTLFunctionConstantValues> {
        unsafe {Self::class().alloc_init(pool).assume_mut() }
    }
    

}