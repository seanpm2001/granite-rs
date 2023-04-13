// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod accel_label;
pub use self::accel_label::AccelLabel;

mod date_picker;
pub use self::date_picker::DatePicker;

mod dialog;
pub use self::dialog::Dialog;

mod header_label;
pub use self::header_label::HeaderLabel;

mod hyper_text_view;
pub use self::hyper_text_view::HyperTextView;

mod message_dialog;
pub use self::message_dialog::MessageDialog;

mod mode_switch;
pub use self::mode_switch::ModeSwitch;

mod overlay_bar;
pub use self::overlay_bar::OverlayBar;

mod placeholder;
pub use self::placeholder::Placeholder;

mod services_contract;
pub use self::services_contract::ServicesContract;

mod services_contractor_proxy;
pub use self::services_contractor_proxy::ServicesContractorProxy;

mod services_settings_serializable;
pub use self::services_settings_serializable::ServicesSettingsSerializable;

mod services_system;
pub use self::services_system::ServicesSystem;

mod settings;
pub use self::settings::Settings;

mod settings_page;
pub use self::settings_page::SettingsPage;

mod settings_sidebar;
pub use self::settings_sidebar::SettingsSidebar;

mod simple_settings_page;
pub use self::simple_settings_page::SimpleSettingsPage;

mod switch_model_button;
pub use self::switch_model_button::SwitchModelButton;

mod time_picker;
pub use self::time_picker::TimePicker;

mod toast;
pub use self::toast::Toast;

mod enums;
pub use self::enums::ServicesContractorError;
pub use self::enums::SettingsColorScheme;
pub use self::enums::SettingsPageStatusType;

pub mod functions;

mod constants;
pub use self::constants::STYLE_CLASS_ACCENT;
#[cfg(any(feature = "v7_1", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v7_1")))]
pub use self::constants::STYLE_CLASS_BACKGROUND;
pub use self::constants::STYLE_CLASS_BACK_BUTTON;
pub use self::constants::STYLE_CLASS_BADGE;
pub use self::constants::STYLE_CLASS_CARD;
pub use self::constants::STYLE_CLASS_CHECKERBOARD;
pub use self::constants::STYLE_CLASS_CIRCULAR;
pub use self::constants::STYLE_CLASS_COLOR_BUTTON;
pub use self::constants::STYLE_CLASS_DEFAULT_DECORATION;
pub use self::constants::STYLE_CLASS_DESTRUCTIVE_ACTION;
pub use self::constants::STYLE_CLASS_DIALOG_CONTENT_AREA;
pub use self::constants::STYLE_CLASS_DIM_LABEL;
pub use self::constants::STYLE_CLASS_ERROR;
pub use self::constants::STYLE_CLASS_FLAT;
#[cfg(any(feature = "v7_1", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v7_1")))]
pub use self::constants::STYLE_CLASS_FRAME;
pub use self::constants::STYLE_CLASS_H1_LABEL;
pub use self::constants::STYLE_CLASS_H2_LABEL;
pub use self::constants::STYLE_CLASS_H3_LABEL;
pub use self::constants::STYLE_CLASS_H4_LABEL;
pub use self::constants::STYLE_CLASS_KEYCAP;
pub use self::constants::STYLE_CLASS_LARGE_ICONS;
pub use self::constants::STYLE_CLASS_LINKED;
pub use self::constants::STYLE_CLASS_MENU;
pub use self::constants::STYLE_CLASS_MENUITEM;
pub use self::constants::STYLE_CLASS_MESSAGE_DIALOG;
pub use self::constants::STYLE_CLASS_MODE_SWITCH;
pub use self::constants::STYLE_CLASS_OSD;
#[cfg(any(feature = "v7_1", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v7_1")))]
pub use self::constants::STYLE_CLASS_RICH_LIST;
pub use self::constants::STYLE_CLASS_ROUNDED;
#[cfg(any(feature = "v7_1", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v7_1")))]
pub use self::constants::STYLE_CLASS_SIDEBAR;
pub use self::constants::STYLE_CLASS_SMALL_LABEL;
pub use self::constants::STYLE_CLASS_SUGGESTED_ACTION;
pub use self::constants::STYLE_CLASS_TEMPERATURE;
pub use self::constants::STYLE_CLASS_TERMINAL;
pub use self::constants::STYLE_CLASS_TITLE_LABEL;
pub use self::constants::STYLE_CLASS_VIEW;
pub use self::constants::STYLE_CLASS_WARMTH;
pub use self::constants::STYLE_CLASS_WARNING;
pub use self::constants::TOOLTIP_SECONDARY_TEXT_MARKUP;

#[doc(hidden)]
pub mod traits {
    pub use super::accel_label::AccelLabelExt;
    pub use super::date_picker::DatePickerExt;
    pub use super::header_label::HeaderLabelExt;
    pub use super::message_dialog::MessageDialogExt;
    pub use super::mode_switch::ModeSwitchExt;
    pub use super::overlay_bar::OverlayBarExt;
    pub use super::placeholder::PlaceholderExt;
    pub use super::services_contract::ServicesContractExt;
    pub use super::services_contractor_proxy::ServicesContractorProxyExt;
    pub use super::services_settings_serializable::ServicesSettingsSerializableExt;
    pub use super::settings::SettingsExt;
    pub use super::settings_page::SettingsPageExt;
    pub use super::settings_sidebar::SettingsSidebarExt;
    pub use super::simple_settings_page::SimpleSettingsPageExt;
    pub use super::switch_model_button::SwitchModelButtonExt;
    pub use super::time_picker::TimePickerExt;
    pub use super::toast::ToastExt;
}
#[doc(hidden)]
pub mod builders {
    pub use super::accel_label::AccelLabelBuilder;
    pub use super::date_picker::DatePickerBuilder;
    pub use super::dialog::DialogBuilder;
    pub use super::header_label::HeaderLabelBuilder;
    pub use super::hyper_text_view::HyperTextViewBuilder;
    pub use super::message_dialog::MessageDialogBuilder;
    pub use super::mode_switch::ModeSwitchBuilder;
    pub use super::overlay_bar::OverlayBarBuilder;
    pub use super::placeholder::PlaceholderBuilder;
    pub use super::settings_sidebar::SettingsSidebarBuilder;
    pub use super::switch_model_button::SwitchModelButtonBuilder;
    pub use super::time_picker::TimePickerBuilder;
    pub use super::toast::ToastBuilder;
}
