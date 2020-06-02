use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use objc_foundation::{object_struct};
use crate::traits;

object_struct!(ITLibArtist);

impl traits::ITLibArtist for ITLibArtist {}
