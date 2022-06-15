use std::ffi::c_void;
use foundationr::NSUInteger;
use objr::bindings::*;
use crate::HasMTLDataType;
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

    For a safe wrapper, see [Self::setConstantValueAtIndex]

    todo: there is a faster path for multiple values of the same type.
    */
    pub unsafe fn setConstantValueTypeAtIndex(&mut self, value: *const c_void, tipe: MTLDataType, atIndex: NSUInteger, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self, Sel::setConstantValue_type_atIndex(), pool, (value, tipe.field(), atIndex))
    }

    /**
    Safe wrapper for [Self::setConstantValueTypeAtIndex].

    This can only be used on a subset of types which conform to [HasMTLDataType].
    */
    pub fn setConstantValueAtIndex<V: ConstantValue>(&mut self, value: &V, atIndex: NSUInteger, pool: &ActiveAutoreleasePool) {
        //safe because arg constrained to ConstantValue, which is known to be of aligned type
        unsafe {
            self.setConstantValueTypeAtIndex(value as *const _ as *const c_void, V::mtl_data_type(), atIndex, pool)
        }
    }
    /**
    Sets the value specified.

    # Safety
    Ensure that the pointer
    * points to valid value
    * has valid alignment
    * points to the type specified

     */
    pub unsafe fn setConstantValueTypeWithName(&mut self, value: *const c_void, tipe: MTLDataType, name: &NSString, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self, Sel::setConstantValue_type_withName(), pool, (value, tipe.field(), name.assume_nonmut_perform()))
    }

    /**
    Safe wrapper for [Self::setConstantValueTypeWithName].

    This can only be used on a subset of types which conform to [HasMTLDataType].
     */
    pub fn setConstantValueWithName<V: ConstantValue>(&mut self, value: &V, name: &NSString, pool: &ActiveAutoreleasePool) {
        //safe because arg constrained to ConstantValue, which is known to be of aligned type
        unsafe {
            self.setConstantValueTypeWithName(value as *const _ as *const c_void, V::mtl_data_type(), name, pool)
        }
    }
}

/**Value can be passed as a constant value.

This is generally HasMTLDataType but has certain restrictions, such as not allowing MTLTextures and similar to be passed.  According
to MSL 5.8.1, these can be "of scalar or vector type".

# Safety:
Must be a type that can be used with [MTLFunctionConstantValues].
*/
pub unsafe trait ConstantValue: HasMTLDataType {}

unsafe impl ConstantValue for u8 {}
unsafe impl ConstantValue for i8 {}
unsafe impl ConstantValue for u16 {}
unsafe impl ConstantValue for i16 {}
unsafe impl ConstantValue for u32 {}
unsafe impl ConstantValue for i32 {}
unsafe impl ConstantValue for u64 {}
unsafe impl ConstantValue for i64 {}
unsafe impl ConstantValue for f32 {}
unsafe impl ConstantValue for bool {}



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
