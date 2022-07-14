use objr::bindings::*;

objc_instance! {
    pub struct MTLResource;
}

objc_selector_group! {
    trait Selectors {
        @selector("label")
        @selector("setLabel:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl MTLResource {
    pub fn label(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSString>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::label(), pool, ());
            NSString::nullable(raw).assume_retained()
        }
    }
    pub fn setLabel(&mut self, label: &NSString, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setLabel_(), pool,(label.assume_nonmut_perform(),))
        }
    }
}