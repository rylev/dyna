// Generated by `wit-bindgen` 0.18.0. DO NOT EDIT!
const _: () = {
  
  #[doc(hidden)]
  #[export_name = "hello"]
  #[allow(non_snake_case)]
  unsafe extern "C" fn __export_hello() {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    
    // Before executing any other code, use this function to run all static
    // constructors, if they have not yet been run. This is a hack required
    // to work around wasi-libc ctors calling import functions to initialize
    // the environment.
    //
    // This functionality will be removed once rust 1.69.0 is stable, at which
    // point wasi-libc will no longer have this behavior.
    //
    // See
    // https://github.com/bytecodealliance/preview2-prototyping/issues/99
    // for more details.
    #[cfg(target_arch="wasm32")]
    wit_bindgen::rt::run_ctors_once();
    
    <_GuestImpl as Guest>::hello();
  }
};
use super::Component as _GuestImpl;
pub trait Guest {
  fn hello();
}
pub mod component {
  pub mod dyna {
    
    #[allow(clippy::all)]
    pub mod dynamic_component {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct Engine{
        handle: wit_bindgen::rt::Resource<Engine>,
      }
      
      impl Engine{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for Engine{
        #[inline]
        unsafe fn drop(_handle: u32) {
          #[cfg(not(target_arch = "wasm32"))]
          unreachable!();
          
          #[cfg(target_arch = "wasm32")]
          {
            #[link(wasm_import_module = "component:dyna/dynamic-component")]
            extern "C" {
              #[link_name = "[resource-drop]engine"]
              fn drop(_: u32);
            }
            
            drop(_handle);
          }
        }
      }
      
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct Component{
        handle: wit_bindgen::rt::Resource<Component>,
      }
      
      impl Component{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for Component{
        #[inline]
        unsafe fn drop(_handle: u32) {
          #[cfg(not(target_arch = "wasm32"))]
          unreachable!();
          
          #[cfg(target_arch = "wasm32")]
          {
            #[link(wasm_import_module = "component:dyna/dynamic-component")]
            extern "C" {
              #[link_name = "[resource-drop]component"]
              fn drop(_: u32);
            }
            
            drop(_handle);
          }
        }
      }
      
      #[derive(Clone, Copy)]
      pub enum TypeItem{
        Str,
      }
      impl ::core::fmt::Debug for TypeItem {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            TypeItem::Str => {
              f.debug_tuple("TypeItem::Str").finish()
            }
          }
        }
      }
      #[derive(Clone)]
      pub struct Function {
        pub params: wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,TypeItem,)>,
        pub results: TypeItem,
      }
      impl ::core::fmt::Debug for Function {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Function").field("params", &self.params).field("results", &self.results).finish()
        }
      }
      #[derive(Clone)]
      pub enum ExportKind{
        Function(Function),
      }
      impl ::core::fmt::Debug for ExportKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            ExportKind::Function(e) => {
              f.debug_tuple("ExportKind::Function").field(e).finish()
            }
          }
        }
      }
      #[derive(Clone)]
      pub struct ExportItem {
        pub name: wit_bindgen::rt::string::String,
        pub kind: ExportKind,
      }
      impl ::core::fmt::Debug for ExportItem {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("ExportItem").field("name", &self.name).field("kind", &self.kind).finish()
        }
      }
      #[derive(Clone)]
      pub enum Val{
        Str(wit_bindgen::rt::string::String),
      }
      impl ::core::fmt::Debug for Val {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            Val::Str(e) => {
              f.debug_tuple("Val::Str").field(e).finish()
            }
          }
        }
      }
      #[derive(Clone)]
      pub enum LoadError{
        InvalidBytes(wit_bindgen::rt::string::String),
      }
      impl ::core::fmt::Debug for LoadError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            LoadError::InvalidBytes(e) => {
              f.debug_tuple("LoadError::InvalidBytes").field(e).finish()
            }
          }
        }
      }
      impl ::core::fmt::Display for LoadError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          write!(f, "{:?}", self)
        }
      }
      
      impl std::error::Error for LoadError {}
      #[derive(Clone, Copy)]
      pub enum CallError{
        NoFunction,
      }
      impl ::core::fmt::Debug for CallError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            CallError::NoFunction => {
              f.debug_tuple("CallError::NoFunction").finish()
            }
          }
        }
      }
      impl ::core::fmt::Display for CallError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          write!(f, "{:?}", self)
        }
      }
      
      impl std::error::Error for CallError {}
      impl Engine {
        #[allow(unused_unsafe, clippy::all)]
        pub fn new() -> Self{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:dyna/dynamic-component")]
            extern "C" {
              #[link_name = "[constructor]engine"]
              fn wit_import() -> i32;
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import() -> i32{ unreachable!() }
            let ret = wit_import();
            Engine::from_handle(ret as u32)
          }
        }
      }
      impl Engine {
        #[allow(unused_unsafe, clippy::all)]
        pub fn load_component(&self,bytes: &[u8],) -> Result<Component,LoadError>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(4))]
            struct RetArea([u8; 16]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = bytes;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:dyna/dynamic-component")]
            extern "C" {
              #[link_name = "[method]engine.load-component"]
              fn wit_import(_: i32, _: i32, _: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, _: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0, len0, ptr1);
            let l2 = i32::from(*((ptr1 + 0) as *const u8));
            match l2 {
              0 => {
                let e = {
                  let l3 = *((ptr1 + 4) as *const i32);
                  
                  Component::from_handle(l3 as u32)
                };
                Ok(e)
              }
              1 => {
                let e = {
                  let l4 = i32::from(*((ptr1 + 4) as *const u8));
                  let v8 = match l4 {
                    n => {
                      debug_assert_eq!(n, 0, "invalid enum discriminant");
                      let e8 = {
                        let l5 = *((ptr1 + 8) as *const i32);
                        let l6 = *((ptr1 + 12) as *const i32);
                        let len7 = l6 as usize;
                        let bytes7 = Vec::from_raw_parts(l5 as *mut _, len7, len7);
                        
                        wit_bindgen::rt::string_lift(bytes7)
                      };
                      LoadError::InvalidBytes(e8)
                    }
                  };
                  
                  v8
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl Component {
        #[allow(unused_unsafe, clippy::all)]
        pub fn call(&self,name: &str,params: &[Val],) -> Result<wit_bindgen::rt::vec::Vec::<Val>,CallError>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(4))]
            struct RetArea([u8; 12]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = name;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec2 = params;
            let len2 = vec2.len() as i32;
            let layout2 = alloc::Layout::from_size_align_unchecked(vec2.len() * 12, 4);
            let result2 = if layout2.size() != 0
            {
              let ptr = alloc::alloc(layout2);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout2);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec2.into_iter().enumerate() {
              let base = result2 as i32 + (i as i32) * 12;
              {
                match e {
                  Val::Str(e) => {
                    *((base + 0) as *mut u8) = (0i32) as u8;
                    let vec1 = e;
                    let ptr1 = vec1.as_ptr() as i32;
                    let len1 = vec1.len() as i32;
                    *((base + 8) as *mut i32) = len1;
                    *((base + 4) as *mut i32) = ptr1;
                  },
                }
              }
            }
            let ptr3 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:dyna/dynamic-component")]
            extern "C" {
              #[link_name = "[method]component.call"]
              fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0, len0, result2 as i32, len2, ptr3);
            let l4 = i32::from(*((ptr3 + 0) as *const u8));
            if layout2.size() != 0 {
              alloc::dealloc(result2, layout2);
            }
            match l4 {
              0 => {
                let e = {
                  let l5 = *((ptr3 + 4) as *const i32);
                  let l6 = *((ptr3 + 8) as *const i32);
                  let base12 = l5;
                  let len12 = l6;
                  let mut result12 = Vec::with_capacity(len12 as usize);
                  for i in 0..len12 {
                    let base = base12 + i * 12;
                    let e12 = {
                      let l7 = i32::from(*((base + 0) as *const u8));
                      let v11 = match l7 {
                        n => {
                          debug_assert_eq!(n, 0, "invalid enum discriminant");
                          let e11 = {
                            let l8 = *((base + 4) as *const i32);
                            let l9 = *((base + 8) as *const i32);
                            let len10 = l9 as usize;
                            let bytes10 = Vec::from_raw_parts(l8 as *mut _, len10, len10);
                            
                            wit_bindgen::rt::string_lift(bytes10)
                          };
                          Val::Str(e11)
                        }
                      };
                      
                      v11
                    };
                    result12.push(e12);
                  }
                  wit_bindgen::rt::dealloc(base12, (len12 as usize) * 12, 4);
                  
                  result12
                };
                Ok(e)
              }
              1 => {
                let e = {
                  let l13 = i32::from(*((ptr3 + 4) as *const u8));
                  let v14 = match l13 {
                    n => {
                      debug_assert_eq!(n, 0, "invalid enum discriminant");
                      CallError::NoFunction
                    }
                  };
                  
                  v14
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl Component {
        #[allow(unused_unsafe, clippy::all)]
        pub fn reflect(&self,) -> wit_bindgen::rt::vec::Vec::<ExportItem>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:dyna/dynamic-component")]
            extern "C" {
              #[link_name = "[method]component.reflect"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = *((ptr0 + 0) as *const i32);
            let l2 = *((ptr0 + 4) as *const i32);
            let base18 = l1;
            let len18 = l2;
            let mut result18 = Vec::with_capacity(len18 as usize);
            for i in 0..len18 {
              let base = base18 + i * 24;
              let e18 = {
                let l3 = *((base + 0) as *const i32);
                let l4 = *((base + 4) as *const i32);
                let len5 = l4 as usize;
                let bytes5 = Vec::from_raw_parts(l3 as *mut _, len5, len5);
                let l6 = i32::from(*((base + 8) as *const u8));
                let v17 = match l6 {
                  n => {
                    debug_assert_eq!(n, 0, "invalid enum discriminant");
                    let e17 = {
                      let l7 = *((base + 12) as *const i32);
                      let l8 = *((base + 16) as *const i32);
                      let base14 = l7;
                      let len14 = l8;
                      let mut result14 = Vec::with_capacity(len14 as usize);
                      for i in 0..len14 {
                        let base = base14 + i * 12;
                        let e14 = {
                          let l9 = *((base + 0) as *const i32);
                          let l10 = *((base + 4) as *const i32);
                          let len11 = l10 as usize;
                          let bytes11 = Vec::from_raw_parts(l9 as *mut _, len11, len11);
                          let l12 = i32::from(*((base + 8) as *const u8));
                          let v13 = match l12 {
                            n => {
                              debug_assert_eq!(n, 0, "invalid enum discriminant");
                              TypeItem::Str
                            }
                          };
                          
                          (wit_bindgen::rt::string_lift(bytes11), v13)
                        };
                        result14.push(e14);
                      }
                      wit_bindgen::rt::dealloc(base14, (len14 as usize) * 12, 4);
                      let l15 = i32::from(*((base + 20) as *const u8));
                      let v16 = match l15 {
                        n => {
                          debug_assert_eq!(n, 0, "invalid enum discriminant");
                          TypeItem::Str
                        }
                      };
                      
                      Function{
                        params: result14,
                        results: v16,
                      }
                    };
                    ExportKind::Function(e17)
                  }
                };
                
                ExportItem{
                  name: wit_bindgen::rt::string_lift(bytes5),
                  kind: v17,
                }
              };
              result18.push(e18);
            }
            wit_bindgen::rt::dealloc(base18, (len18 as usize) * 24, 4);
            result18
          }
        }
      }
      
    }
    
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:guest"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 660] = [0, 97, 115, 109, 13, 0, 1, 0, 0, 25, 22, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 45, 101, 110, 99, 111, 100, 105, 110, 103, 4, 0, 7, 153, 4, 1, 65, 2, 1, 65, 4, 1, 66, 35, 4, 0, 6, 101, 110, 103, 105, 110, 101, 3, 1, 4, 0, 9, 99, 111, 109, 112, 111, 110, 101, 110, 116, 3, 1, 1, 113, 1, 3, 115, 116, 114, 0, 0, 4, 0, 9, 116, 121, 112, 101, 45, 105, 116, 101, 109, 3, 0, 2, 1, 111, 2, 115, 3, 1, 112, 4, 1, 114, 2, 6, 112, 97, 114, 97, 109, 115, 5, 7, 114, 101, 115, 117, 108, 116, 115, 3, 4, 0, 8, 102, 117, 110, 99, 116, 105, 111, 110, 3, 0, 6, 1, 113, 1, 8, 102, 117, 110, 99, 116, 105, 111, 110, 1, 7, 0, 4, 0, 11, 101, 120, 112, 111, 114, 116, 45, 107, 105, 110, 100, 3, 0, 8, 1, 114, 2, 4, 110, 97, 109, 101, 115, 4, 107, 105, 110, 100, 9, 4, 0, 11, 101, 120, 112, 111, 114, 116, 45, 105, 116, 101, 109, 3, 0, 10, 1, 113, 1, 3, 115, 116, 114, 1, 115, 0, 4, 0, 3, 118, 97, 108, 3, 0, 12, 1, 113, 1, 13, 105, 110, 118, 97, 108, 105, 100, 45, 98, 121, 116, 101, 115, 1, 115, 0, 4, 0, 10, 108, 111, 97, 100, 45, 101, 114, 114, 111, 114, 3, 0, 14, 1, 113, 1, 11, 110, 111, 45, 102, 117, 110, 99, 116, 105, 111, 110, 0, 0, 4, 0, 10, 99, 97, 108, 108, 45, 101, 114, 114, 111, 114, 3, 0, 16, 1, 105, 0, 1, 64, 0, 0, 18, 4, 0, 19, 91, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 93, 101, 110, 103, 105, 110, 101, 1, 19, 1, 104, 0, 1, 112, 125, 1, 105, 1, 1, 106, 1, 22, 1, 15, 1, 64, 2, 4, 115, 101, 108, 102, 20, 5, 98, 121, 116, 101, 115, 21, 0, 23, 4, 0, 29, 91, 109, 101, 116, 104, 111, 100, 93, 101, 110, 103, 105, 110, 101, 46, 108, 111, 97, 100, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 1, 24, 1, 104, 1, 1, 112, 13, 1, 106, 1, 26, 1, 17, 1, 64, 3, 4, 115, 101, 108, 102, 25, 4, 110, 97, 109, 101, 115, 6, 112, 97, 114, 97, 109, 115, 26, 0, 27, 4, 0, 22, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 109, 112, 111, 110, 101, 110, 116, 46, 99, 97, 108, 108, 1, 28, 1, 112, 11, 1, 64, 1, 4, 115, 101, 108, 102, 25, 0, 29, 4, 0, 25, 91, 109, 101, 116, 104, 111, 100, 93, 99, 111, 109, 112, 111, 110, 101, 110, 116, 46, 114, 101, 102, 108, 101, 99, 116, 1, 30, 3, 1, 32, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 100, 121, 110, 97, 47, 100, 121, 110, 97, 109, 105, 99, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 0, 1, 64, 0, 1, 0, 4, 0, 5, 104, 101, 108, 108, 111, 1, 1, 4, 1, 21, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 103, 117, 101, 115, 116, 47, 103, 117, 101, 115, 116, 4, 0, 11, 11, 1, 0, 5, 103, 117, 101, 115, 116, 3, 0, 0, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 50, 49, 46, 48, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 56, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
