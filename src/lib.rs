use std::any::TypeId;
use std::marker::PhantomData;

pub use dynamic_dispatch_proc_macro::dynamic_dispatch;

#[derive(Debug, PartialEq, Eq)]
pub struct DynamicDispatch<T: ?Sized + 'static> {
    pub value: TypeId,
    pub _phantom: PhantomData<&'static T>,
}
