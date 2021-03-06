pub use crate::prelude::*;
use crate::image::Image;
use fltk_sys::widget::*;
use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw;

/// An abstract type, shouldn't be instantiated in user code
#[derive(WidgetExt, Debug)]
pub struct Widget {
    _inner: *mut Fl_Widget,
}

// /// A conversion function for internal use
// impl<W: WidgetExt> From<W> for Widget {
//     fn from(s: W) -> Self {
//         let widg: *mut Fl_Widget = s.as_widget_ptr();
//         Widget { _inner: widg }
//     }
// }

impl Widget {
    /// Initialize a Widget base from a pointer
    pub unsafe fn from_raw(ptr: *mut Fl_Widget) -> Self {
        assert!(!ptr.is_null());
        Widget { _inner: ptr }
    }
    
    /// Returns the inner pointer
    pub unsafe fn as_ptr(&self) -> *mut Fl_Widget {
        self._inner
    }
    
    /// Transform Widget base to another Widget
    pub fn into<W: WidgetExt>(self) -> W {
        unsafe { W::from_widget_ptr(self._inner) }
    }
}
