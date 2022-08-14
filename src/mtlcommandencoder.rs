use objr::bindings::*;
objc_instance! {
    pub struct MTLCommandEncoder;
}
objc_selector_group! {
    trait Selectors {
        @selector("label")
        @selector("setLabel:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLCommandEncoder {
    pub fn label(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSString>> {
        unsafe {
            let r = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::label(), pool, ());
            NSString::nullable(r).assume_retained()
        }
    }
    pub fn setLabel(&mut self, value: &NSString, pool: &ActiveAutoreleasePool) {
        unsafe{Self::perform_primitive(self, Sel::setLabel_(), pool, (value.assume_nonmut_perform(),))}
    }
}
