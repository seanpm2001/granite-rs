// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GraniteHyperTextView")]
    pub struct HyperTextView(Object<ffi::GraniteHyperTextView, ffi::GraniteHyperTextViewClass>) @extends gtk::TextView, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;

    match fn {
        type_ => || ffi::granite_hyper_text_view_get_type(),
    }
}

impl HyperTextView {
        pub const NONE: Option<&'static HyperTextView> = None;
    

    #[doc(alias = "granite_hyper_text_view_new")]
    pub fn new() -> HyperTextView {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::granite_hyper_text_view_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`HyperTextView`] objects.
            ///
            /// This method returns an instance of [`HyperTextViewBuilder`](crate::builders::HyperTextViewBuilder) which can be used to create [`HyperTextView`] objects.
            pub fn builder() -> HyperTextViewBuilder {
                HyperTextViewBuilder::new()
            }
        
}

impl Default for HyperTextView {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`HyperTextView`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct HyperTextViewBuilder {
            builder: glib::object::ObjectBuilder<'static, HyperTextView>,
        }

        impl HyperTextViewBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn accepts_tab(self, accepts_tab: bool) -> Self {
                            Self { builder: self.builder.property("accepts-tab", accepts_tab), }
                        }

                            pub fn bottom_margin(self, bottom_margin: i32) -> Self {
                            Self { builder: self.builder.property("bottom-margin", bottom_margin), }
                        }

                            //pub fn buffer(self, buffer: &impl IsA</*Ignored*/gtk::TextBuffer>) -> Self {
                        //    Self { builder: self.builder.property("buffer", buffer.clone().upcast()), }
                        //}

                            pub fn cursor_visible(self, cursor_visible: bool) -> Self {
                            Self { builder: self.builder.property("cursor-visible", cursor_visible), }
                        }

                            pub fn editable(self, editable: bool) -> Self {
                            Self { builder: self.builder.property("editable", editable), }
                        }

                            //pub fn extra_menu(self, extra_menu: &impl IsA</*Ignored*/gio::MenuModel>) -> Self {
                        //    Self { builder: self.builder.property("extra-menu", extra_menu.clone().upcast()), }
                        //}

                            pub fn im_module(self, im_module: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("im-module", im_module.into()), }
                        }

                            pub fn indent(self, indent: i32) -> Self {
                            Self { builder: self.builder.property("indent", indent), }
                        }

                            //pub fn input_hints(self, input_hints: /*Ignored*/gtk::InputHints) -> Self {
                        //    Self { builder: self.builder.property("input-hints", input_hints), }
                        //}

                            //pub fn input_purpose(self, input_purpose: /*Ignored*/gtk::InputPurpose) -> Self {
                        //    Self { builder: self.builder.property("input-purpose", input_purpose), }
                        //}

                            //pub fn justification(self, justification: /*Ignored*/gtk::Justification) -> Self {
                        //    Self { builder: self.builder.property("justification", justification), }
                        //}

                            pub fn left_margin(self, left_margin: i32) -> Self {
                            Self { builder: self.builder.property("left-margin", left_margin), }
                        }

                            pub fn monospace(self, monospace: bool) -> Self {
                            Self { builder: self.builder.property("monospace", monospace), }
                        }

                            pub fn overwrite(self, overwrite: bool) -> Self {
                            Self { builder: self.builder.property("overwrite", overwrite), }
                        }

                            pub fn pixels_above_lines(self, pixels_above_lines: i32) -> Self {
                            Self { builder: self.builder.property("pixels-above-lines", pixels_above_lines), }
                        }

                            pub fn pixels_below_lines(self, pixels_below_lines: i32) -> Self {
                            Self { builder: self.builder.property("pixels-below-lines", pixels_below_lines), }
                        }

                            pub fn pixels_inside_wrap(self, pixels_inside_wrap: i32) -> Self {
                            Self { builder: self.builder.property("pixels-inside-wrap", pixels_inside_wrap), }
                        }

                            pub fn right_margin(self, right_margin: i32) -> Self {
                            Self { builder: self.builder.property("right-margin", right_margin), }
                        }

                            //pub fn tabs(self, tabs: /*Ignored*/&pango::TabArray) -> Self {
                        //    Self { builder: self.builder.property("tabs", tabs), }
                        //}

                            pub fn top_margin(self, top_margin: i32) -> Self {
                            Self { builder: self.builder.property("top-margin", top_margin), }
                        }

                            //pub fn wrap_mode(self, wrap_mode: /*Ignored*/gtk::WrapMode) -> Self {
                        //    Self { builder: self.builder.property("wrap-mode", wrap_mode), }
                        //}

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            pub fn can_target(self, can_target: bool) -> Self {
                            Self { builder: self.builder.property("can-target", can_target), }
                        }

                            pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
                            Self { builder: self.builder.property("css-classes", css_classes.into()), }
                        }

                            pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("css-name", css_name.into()), }
                        }

                            //pub fn cursor(self, cursor: /*Ignored*/&gdk::Cursor) -> Self {
                        //    Self { builder: self.builder.property("cursor", cursor), }
                        //}

                            pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                            pub fn focusable(self, focusable: bool) -> Self {
                            Self { builder: self.builder.property("focusable", focusable), }
                        }

                            //pub fn halign(self, halign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("halign", halign), }
                        //}

                            pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            pub fn hexpand(self, hexpand: bool) -> Self {
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            //pub fn layout_manager(self, layout_manager: &impl IsA</*Ignored*/gtk::LayoutManager>) -> Self {
                        //    Self { builder: self.builder.property("layout-manager", layout_manager.clone().upcast()), }
                        //}

                            pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            pub fn margin_end(self, margin_end: i32) -> Self {
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            pub fn margin_start(self, margin_start: i32) -> Self {
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            pub fn margin_top(self, margin_top: i32) -> Self {
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn opacity(self, opacity: f64) -> Self {
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            //pub fn overflow(self, overflow: /*Ignored*/gtk::Overflow) -> Self {
                        //    Self { builder: self.builder.property("overflow", overflow), }
                        //}

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                            //pub fn valign(self, valign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("valign", valign), }
                        //}

                            pub fn vexpand(self, vexpand: bool) -> Self {
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

                            //pub fn accessible_role(self, accessible_role: /*Ignored*/gtk::AccessibleRole) -> Self {
                        //    Self { builder: self.builder.property("accessible-role", accessible_role), }
                        //}

                            //pub fn hadjustment(self, hadjustment: &impl IsA</*Ignored*/gtk::Adjustment>) -> Self {
                        //    Self { builder: self.builder.property("hadjustment", hadjustment.clone().upcast()), }
                        //}

                            //pub fn hscroll_policy(self, hscroll_policy: /*Ignored*/gtk::ScrollablePolicy) -> Self {
                        //    Self { builder: self.builder.property("hscroll-policy", hscroll_policy), }
                        //}

                            //pub fn vadjustment(self, vadjustment: &impl IsA</*Ignored*/gtk::Adjustment>) -> Self {
                        //    Self { builder: self.builder.property("vadjustment", vadjustment.clone().upcast()), }
                        //}

                            //pub fn vscroll_policy(self, vscroll_policy: /*Ignored*/gtk::ScrollablePolicy) -> Self {
                        //    Self { builder: self.builder.property("vscroll-policy", vscroll_policy), }
                        //}

    // rustdoc-stripper-ignore-next
    /// Build the [`HyperTextView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> HyperTextView {
    self.builder.build() }
}

impl fmt::Display for HyperTextView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HyperTextView")
    }
}
