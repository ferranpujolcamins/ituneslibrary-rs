use objc_foundation::{INSObject, NSString, INSString};
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;

pub trait ITLibArtist: INSObject {

    fn name(&self) -> Option<String> {
        unsafe {
            let name: NSString = msg_send![self, name];
            Some(String::from(name.as_str()))
        }
    }

    fn sort_name(&self) -> Option<String> {
        unsafe {
            let name: NSString = msg_send![self, sortName];
            Some(String::from(name.as_str()))
        }
    }

    fn persistent_id(&self) -> u64 {
        unsafe {
            msg_send![self, persistentId]
        }
    }
}
