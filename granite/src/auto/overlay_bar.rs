// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GraniteOverlayBar")]
    pub struct OverlayBar(Object<ffi::GraniteOverlayBar, ffi::GraniteOverlayBarClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::granite_overlay_bar_get_type(),
    }
}

impl OverlayBar {
    pub const NONE: Option<&'static OverlayBar> = None;

    #[doc(alias = "granite_overlay_bar_new")]
    pub fn new(overlay: Option<&gtk::Overlay>) -> OverlayBar {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::granite_overlay_bar_new(overlay.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`OverlayBar`] objects.
    ///
    /// This method returns an instance of [`OverlayBarBuilder`](crate::builders::OverlayBarBuilder) which can be used to create [`OverlayBar`] objects.
    pub fn builder() -> OverlayBarBuilder {
        OverlayBarBuilder::new()
    }
}

impl Default for OverlayBar {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`OverlayBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct OverlayBarBuilder {
    builder: glib::object::ObjectBuilder<'static, OverlayBar>,
}

impl OverlayBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn overlay(self, overlay: &gtk::Overlay) -> Self {
        Self {
            builder: self.builder.property("overlay", overlay.clone()),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    //pub fn layout_manager(self, layout_manager: &impl IsA</*Ignored*/gtk::LayoutManager>) -> Self {
    //    Self { builder: self.builder.property("layout-manager", layout_manager.clone().upcast()), }
    //}

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    //pub fn accessible_role(self, accessible_role: /*Ignored*/gtk::AccessibleRole) -> Self {
    //    Self { builder: self.builder.property("accessible-role", accessible_role), }
    //}

    // rustdoc-stripper-ignore-next
    /// Build the [`OverlayBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> OverlayBar {
        self.builder.build()
    }
}

pub trait OverlayBarExt: 'static {
    #[doc(alias = "granite_overlay_bar_get_overlay")]
    #[doc(alias = "get_overlay")]
    fn overlay(&self) -> Option<gtk::Overlay>;

    #[doc(alias = "granite_overlay_bar_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> Option<glib::GString>;

    #[doc(alias = "granite_overlay_bar_set_label")]
    fn set_label(&self, value: &str);

    #[doc(alias = "granite_overlay_bar_get_active")]
    #[doc(alias = "get_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "granite_overlay_bar_set_active")]
    fn set_active(&self, value: bool);

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<OverlayBar>> OverlayBarExt for O {
    fn overlay(&self) -> Option<gtk::Overlay> {
        unsafe {
            from_glib_none(ffi::granite_overlay_bar_get_overlay(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::granite_overlay_bar_get_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_label(&self, value: &str) {
        unsafe {
            ffi::granite_overlay_bar_set_label(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::granite_overlay_bar_get_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_active(&self, value: bool) {
        unsafe {
            ffi::granite_overlay_bar_set_active(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<OverlayBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GraniteOverlayBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(OverlayBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P: IsA<OverlayBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GraniteOverlayBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(OverlayBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for OverlayBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OverlayBar")
    }
}
