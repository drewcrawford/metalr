use objr::bindings::*;
use super::MTLFunction;
use crate::MTLFunctionConstantValues;
use blocksr::once_escaping;
use std::future::Future;
objc_instance! {
    pub struct MTLLibrary;
}
objc_selector_group! {
    trait MTLLibrarySelectors {
        @selector("newFunctionWithName:")
        @selector("newFunctionWithName:constantValues:completionHandler:")
    }
    impl MTLLibrarySelectors for Sel{}
}

once_escaping!(NewFunctionCompletionHandler(function: *const MTLFunction, error: *const NSError) -> ());
unsafe impl Arguable for &NewFunctionCompletionHandler {}
#[allow(non_snake_case)]
impl MTLLibrary {
    pub fn newFunctionWithName(&mut self, name: &NSString, pool: &ActiveAutoreleasePool) -> Option<StrongCell<MTLFunction>> {
        unsafe {
            let ptr = Self::perform(self, Sel::newFunctionWithName_(), pool, (name,));
            MTLFunction::nullable(ptr).assume_retained()
        }
    }
    pub fn newFunctionWithNameConstantValuesCompletionHandler<F>(&self,name: &NSString, constantValues: &MTLFunctionConstantValues, completionHandler: F, pool: &ActiveAutoreleasePool)
    where F: FnOnce(Result<&MTLFunction,&NSError>) + Send + 'static {
        let block = unsafe{ NewFunctionCompletionHandler::new(|function, error| {
           let result;
            if function.is_null() {
               result = Err(&*error)
           }
            else {
                result = Ok(&*function)
            }
            completionHandler(result)
        })};
        unsafe {
            //I'm pretty confident it's ok to do this nonmutably.  Maybe consider this for the other fns as well?
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::newFunctionWithName_constantValues_completionHandler(), pool, (name, constantValues, &block))
        }
    }

    pub fn newFunctionAsync(&self, name: &NSString, constantValues: &MTLFunctionConstantValues, pool: &ActiveAutoreleasePool) -> impl Future<Output=Result<StrongCell<MTLFunction>,StrongCell<NSError>>> {
        let (continuation, completer) = blocksr::continuation::Continuation::<(),Result<StrongCell<MTLFunction>,StrongCell<NSError>>>::new();
        self.newFunctionWithNameConstantValuesCompletionHandler(name, constantValues, |result| {
            let result = result.map(|r| StrongCell::retaining(r)).map_err(|e| StrongCell::retaining(e));
            completer.complete(result);
        }, pool);
        continuation
    }
}

