use objr::bindings::*;
objc_class! {
    pub struct MTLFunctionConstantValues {
        @class(MTLFunctionConstantValues)
    }
}

impl MTLFunctionConstantValues {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongCell<MTLFunctionConstantValues> {
        Self::class().alloc_init(pool)
    }
}