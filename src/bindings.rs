#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_hash_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::hash(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    _rt::as_i32(result1)
}
pub trait Guest {
    fn hash(input: _rt::Vec<u8>) -> u32;
}
#[doc(hidden)]
macro_rules! __export_world_crc32_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "hash"] unsafe extern "C" fn export_hash(arg0 :
        * mut u8, arg1 : usize,) -> i32 { $($path_to_types)*:: _export_hash_cabi::<$ty >
        (arg0, arg1) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_crc32_cabi;
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod crc32_wasm {
            #[allow(dead_code, clippy::all)]
            pub mod crc32_hasher {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Hasher {
                    handle: _rt::Resource<Hasher>,
                }
                type _HasherRep<T> = Option<T>;
                impl Hasher {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Hasher`.
                    pub fn new<T: GuestHasher>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _HasherRep<T> = Some(val);
                        let ptr: *mut _HasherRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestHasher>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestHasher>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestHasher>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _HasherRep<T>);
                    }
                    fn as_ptr<T: GuestHasher>(&self) -> *mut _HasherRep<T> {
                        Hasher::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Hasher`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct HasherBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Hasher>,
                }
                impl<'a> HasherBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestHasher>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _HasherRep<T> {
                        Hasher::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Hasher {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:crc32-wasm/crc32-hasher"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]hasher"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_hasher_cabi<T: GuestHasher>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = Hasher::new(T::new());
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_hasher_update_cabi<T: GuestHasher>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    T::update(
                        HasherBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_hasher_finalize_cabi<T: GuestHasher>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::finalize(
                        HasherBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    type Hasher: GuestHasher;
                }
                pub trait GuestHasher: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:crc32-wasm/crc32-hasher"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]hasher"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:crc32-wasm/crc32-hasher"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]hasher"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new() -> Self;
                    fn update(&self, input: _rt::Vec<u8>);
                    fn finalize(&self) -> u32;
                }
                #[doc(hidden)]
                macro_rules! __export_component_crc32_wasm_crc32_hasher_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:crc32-wasm/crc32-hasher#[constructor]hasher"] unsafe
                        extern "C" fn export_constructor_hasher() -> i32 {
                        $($path_to_types)*:: _export_constructor_hasher_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Hasher > () } #[export_name =
                        "component:crc32-wasm/crc32-hasher#[method]hasher.update"] unsafe
                        extern "C" fn export_method_hasher_update(arg0 : * mut u8, arg1 :
                        * mut u8, arg2 : usize,) { $($path_to_types)*::
                        _export_method_hasher_update_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Hasher > (arg0, arg1, arg2) } #[export_name =
                        "component:crc32-wasm/crc32-hasher#[method]hasher.finalize"]
                        unsafe extern "C" fn export_method_hasher_finalize(arg0 : * mut
                        u8,) -> i32 { $($path_to_types)*::
                        _export_method_hasher_finalize_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Hasher > (arg0) } const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "component:crc32-wasm/crc32-hasher#[dtor]hasher"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Hasher::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Hasher > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_crc32_wasm_crc32_hasher_cabi;
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::boxed::Box;
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_crc32_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_crc32_cabi!($ty with_types_in
        $($path_to_types_root)*); $($path_to_types_root)*::
        exports::component::crc32_wasm::crc32_hasher::__export_component_crc32_wasm_crc32_hasher_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::crc32_wasm::crc32_hasher);
    };
}
#[doc(inline)]
pub(crate) use __export_crc32_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:crc32:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 358] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xea\x01\x01A\x02\x01\
A\x05\x01p}\x01@\x01\x05input\0\0y\x04\0\x04hash\x01\x01\x01B\x0a\x04\0\x06hashe\
r\x03\x01\x01i\0\x01@\0\0\x01\x04\0\x13[constructor]hasher\x01\x02\x01h\0\x01p}\x01\
@\x02\x04self\x03\x05input\x04\x01\0\x04\0\x15[method]hasher.update\x01\x05\x01@\
\x01\x04self\x03\0y\x04\0\x17[method]hasher.finalize\x01\x06\x04\x01!component:c\
rc32-wasm/crc32-hasher\x05\x02\x04\x01\x1acomponent:crc32-wasm/crc32\x04\0\x0b\x0b\
\x01\0\x05crc32\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\
\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
