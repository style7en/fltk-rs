/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Window {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Window_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Window;
}
extern "C" {
    pub fn Fl_Window_x(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_y(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_width(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_height(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_label(arg1: *mut Fl_Window) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Window_set_label(arg1: *mut Fl_Window, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Window_redraw(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_show(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_hide(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_activate(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_deactivate(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_redraw_label(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_resize(
        arg1: *mut Fl_Window,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Window_tooltip(arg1: *mut Fl_Window) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Window_set_tooltip(arg1: *mut Fl_Window, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Window_get_type(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_type(arg1: *mut Fl_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_color(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_color(arg1: *mut Fl_Window, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_label_color(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_label_color(arg1: *mut Fl_Window, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_label_font(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_label_font(arg1: *mut Fl_Window, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_label_size(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_label_size(arg1: *mut Fl_Window, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_label_type(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_label_type(arg1: *mut Fl_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_box(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_box(arg1: *mut Fl_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_changed(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_changed(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_clear_changed(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_align(arg1: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_set_align(arg1: *mut Fl_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_delete(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_set_image(arg1: *mut Fl_Window, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Window_begin(self_: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_end(self_: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_find(
        self_: *mut Fl_Window,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_add(self_: *mut Fl_Window, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Window_insert(
        self_: *mut Fl_Window,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Window_remove(self_: *mut Fl_Window, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Window_clear(self_: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_children(self_: *mut Fl_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Window_make_resizable(self_: *mut Fl_Window, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Window_make_modal(arg1: *mut Fl_Window, boolean: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Window_fullscreen(arg1: *mut Fl_Window, boolean: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Window_make_current(arg1: *mut Fl_Window);
}
extern "C" {
    pub fn Fl_Window_set_icon(arg1: *mut Fl_Window, arg2: *const ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Double_Window {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Double_Window_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Double_Window;
}
extern "C" {
    pub fn Fl_Double_Window_x(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_y(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_width(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_height(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_label(arg1: *mut Fl_Double_Window) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Double_Window_set_label(
        arg1: *mut Fl_Double_Window,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Double_Window_redraw(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_show(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_hide(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_activate(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_deactivate(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_redraw_label(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_resize(
        arg1: *mut Fl_Double_Window,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Double_Window_tooltip(arg1: *mut Fl_Double_Window) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Double_Window_set_tooltip(
        arg1: *mut Fl_Double_Window,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Double_Window_get_type(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_type(arg1: *mut Fl_Double_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_color(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_color(arg1: *mut Fl_Double_Window, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_label_color(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_label_color(
        arg1: *mut Fl_Double_Window,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Double_Window_label_font(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_label_font(
        arg1: *mut Fl_Double_Window,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Double_Window_label_size(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_label_size(arg1: *mut Fl_Double_Window, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_label_type(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_label_type(arg1: *mut Fl_Double_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_box(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_box(arg1: *mut Fl_Double_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_changed(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_changed(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_clear_changed(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_align(arg1: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_set_align(arg1: *mut Fl_Double_Window, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_delete(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_set_image(
        arg1: *mut Fl_Double_Window,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Double_Window_begin(self_: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_end(self_: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_find(
        self_: *mut Fl_Double_Window,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_add(self_: *mut Fl_Double_Window, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Double_Window_insert(
        self_: *mut Fl_Double_Window,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Double_Window_remove(self_: *mut Fl_Double_Window, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Double_Window_clear(self_: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_children(self_: *mut Fl_Double_Window) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Double_Window_make_resizable(
        self_: *mut Fl_Double_Window,
        arg1: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Double_Window_make_modal(
        arg1: *mut Fl_Double_Window,
        boolean: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Double_Window_fullscreen(
        arg1: *mut Fl_Double_Window,
        boolean: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Double_Window_make_current(arg1: *mut Fl_Double_Window);
}
extern "C" {
    pub fn Fl_Double_Window_set_icon(
        arg1: *mut Fl_Double_Window,
        arg2: *const ::std::os::raw::c_void,
    );
}
