use foundationr::NSUInteger;
use objr::bindings::*;

objc_enum! {
    #[derive(PartialEq,Debug)]
    pub struct MTLCompareFunction<NSUInteger>;
    impl MTLCompareFunction {
        Never = 0,
        Less = 1,
        Equal = 2,
        LessEqual = 3,
        Greater = 4,
        NotEqual = 5,
        GreaterEqual = 6,
        Always = 7
    }
}

objc_class! {
    pub struct MTLDepthStencilDescriptor {
        @class(MTLDepthStencilDescriptor)
    }
}

objc_selector_group! {
    trait Selectors {
        @selector("depthCompareFunction")
        @selector("setDepthCompareFunction:")
        @selector("isDepthWriteEnabled")
        @selector("setDepthWriteEnabled:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLDepthStencilDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe{Self::class().alloc_init(pool).assume_mut()}
    }
    pub fn depthCompareFunction(&self, pool: &ActiveAutoreleasePool)  -> MTLCompareFunction {
        unsafe {
            MTLCompareFunction(Self::perform_primitive(self.assume_nonmut_perform(), Sel::depthCompareFunction(),pool, ()))
        }
    }
    pub fn set_depthCompareFunction(&mut self, function: MTLCompareFunction, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDepthCompareFunction_(), pool, (function.field(),))
        }
    }
    pub fn isDepthWriteEnabled(&self, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::isDepthWriteEnabled(), pool, ())
        }
    }
    pub fn set_depthWriteEnabled(&mut self, enabled: bool, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDepthWriteEnabled_(), pool, (enabled,))
        }
    }

}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let mut descriptor = MTLDepthStencilDescriptor::new(pool);
        descriptor.set_depthCompareFunction(MTLCompareFunction::Equal, pool);
        let read_back = descriptor.depthCompareFunction(pool);
        assert_eq!(read_back, MTLCompareFunction::Equal);

        assert_eq!(descriptor.isDepthWriteEnabled(pool),false);
        descriptor.set_depthWriteEnabled(true, pool);
        assert_eq!(descriptor.isDepthWriteEnabled(pool),true);

    })
}