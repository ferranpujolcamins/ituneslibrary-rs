extern crate objc;

use std::result::Result;
use objc::runtime::{Object, Sel, BOOL};
use objc::{msg_send, class, sel, sel_impl, __send_message, Encode, Encoding};
use objc::rc::StrongPtr;
use std::os::raw::c_void;

#[link(name = "iTunesLibrary", kind = "framework")]
extern {}

struct ITLibrary {
    obj: *mut Object
}

impl ITLibrary {

    fn new(api_version: &str, options: ITLibInitOptions) -> Result<Self, ()> {
        unsafe {
            let obj: *mut Object = msg_send![class!(ITLibrary), alloc];
            let version = nsstring("1.0");
            let error: *mut Object = msg_send![class!(NSError), alloc];
            let error: *mut *mut Object = &mut error;
            let obj: *mut Object = msg_send![obj, initWithAPIVersion: version error: error];
            assert!(!obj.is_null());
            return Ok(ITLibrary{ obj })
        }
    }

    fn reload_data(&self) {
        unsafe {
            let _: BOOL = msg_send![self.obj, reloadData];
        }
    }

    fn unload_data(&self) {
        unsafe {
            let _: () = msg_send![self.obj, unloadData];
        }
    }
}

enum ITLibInitOptions {
    None,
    LazyLoadData
}

fn nsstring(string: &str) -> *mut Object {
    const UTF8_ENCODING: usize = 4;
    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, initWithBytes:bytes
                                                       length:string.len()
                                                     encoding:UTF8_ENCODING];
        return obj
    }
}

#[cfg(test)]
mod tests {
    use crate::{ITLibrary, ITLibInitOptions};

    #[test]
    fn it_works() {
        let library = ITLibrary::new(
            "1.0",
            ITLibInitOptions::None
        ).ok().unwrap();
        // library.reload_data();
        library.unload_data();
        assert!(true);
    }
}
