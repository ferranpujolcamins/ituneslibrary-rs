extern crate ituneslibrary_sys;
extern crate objc;
extern crate objc_foundation;
extern crate chrono;

#[link(name = "iTunesLibrary", kind = "framework")]
extern {}

mod album;
mod artist;
mod artwork;
mod error;
mod library;
mod media_item;
mod traits;

use std::os::raw::c_void;
use objc::runtime::{Object, Sel, BOOL};
use objc_foundation::{INSArray, NSArray, NSString, INSString, NSObject};
use objc::{msg_send, class, sel, sel_impl, __send_message, Encode, Encoding};
use objc_id::{Owned, Id};
use ituneslibrary_sys::{id};
use std::any::Any;
use std::ops::Deref;
use std::mem;
use chrono::{Date, Utc};
use url::Url;


fn t() {
    // unsafe {
    //     let version = nsstring("1.0");
    //     let obj: *mut Object = msg_send![class!(ITLibrary), alloc];
    //     let mut error: *mut Object = msg_send![class!(NSError), alloc];
    //     let mut error2: *mut *mut Object = &mut error;
    //     let error3: *mut *mut *mut Object = &mut error2;
    //     let obc: *mut Object = obj.initWithAPIVersion_error_(
    //         version,
    //         error3
    //     );
    //     let v = obj.apiMajorVersion();
    //     println!("api version: {}", v);
    //
    //     // let a = NSArray:: obj.all_media_items() as &INSArray<Item = Any, Own = Owned>;
    //     let a: *mut NSArray<NSObject> = mem::transmute(obj.allMediaItems());
    //     let c = (*a).count();
    //     let z = &(*a)[2];
    //     println!("count: {}", (*a).count())
    // }
}

unsafe fn from_obj_array<A>(obj_array: *mut Object) -> Id<A> where A: INSArray {
    let cls = A::class();
    let c: *mut Object = msg_send![obj_array, class];
    let obj: *mut A = msg_send![cls, arrayWithArray: obj_array];
    Id::from_retained_ptr(obj)
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
    use crate::{t};
    use objc_id::ShareId;
    use crate::traits;
    use crate::library::ITLibrary;
    use crate::media_item::ITLibMediaItem;
    use crate::traits::ITLibraryTrait;

    #[test]
    fn it_works() {
        let library = ITLibrary::new_with_api_version("1.0").unwrap();
        let items: Vec<ShareId<ITLibMediaItem>> = library.all_media_items();
        println!("OK")
    }
}
