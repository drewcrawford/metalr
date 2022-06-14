use std::ffi::c_void;
use foundationr::NSUInteger;
use objr::bindings::*;
use crate::mtlargument::MTLDataType;
objc_class! {
    pub struct MTLFunctionConstantValues {
        @class(MTLFunctionConstantValues)
    }
}
objc_selector_group! {
    trait S {
        @selector("setConstantValue:type:atIndex:")
        @selector("setConstantValue:type:withName:")
    }
    impl S for Sel {}
}

#[allow(non_snake_case)]
impl MTLFunctionConstantValues {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<MTLFunctionConstantValues> {
        unsafe {Self::class().alloc_init(pool).assume_mut() }
    }
    /**
    Sets the value specified.

    # Safety
    Ensure that the pointer
    * points to valid value
    * has valid alignment
    * points to the type specified

    For a safe(r) wrapper, see setConstantValueAtIndex.

    todo: there is a faster path for multiple values of the same type.
    */
    pub unsafe fn setConstantValueTypeAtIndex(&mut self, value: *const c_void, tipe: MTLDataType, atIndex: NSUInteger, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self, Sel::setConstantValue_type_atIndex(), pool, (value, tipe.field(), atIndex))
    }
    /**
    Sets the value specified.

    # Safety
    Ensure that the pointer
    * points to valid value
    * has valid alignment
    * points to the type specified

    For a safe(r) wrapper, see setConstantValueAtIndex.

    todo: there is a faster path for multiple values of the same type.
     */
    pub unsafe fn setConstantValueTypeWithName(&mut self, value: *const c_void, tipe: MTLDataType, name: &NSString, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self, Sel::setConstantValue_type_withName(), pool, (value, tipe.field(), name.assume_nonmut_perform()))
    }
}

#[test] fn test_raw() {
    autoreleasepool(|pool| {
        let mut f = MTLFunctionConstantValues::new(pool);
        unsafe {
            f.setConstantValueTypeAtIndex(&(23 as u8) as *const _ as *const c_void, MTLDataType::Char, 1337, pool);
            f.setConstantValueTypeWithName(&true as *const _ as *const c_void, MTLDataType::Bool, objc_nsstring!("myParameter"), pool);
            let description = f.description(pool).to_string();
            assert!(description.contains("23"));
            assert!(description.contains("1337"));
            assert!(description.contains("myParameter"));
        }
    })
}