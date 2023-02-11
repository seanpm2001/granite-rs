// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../..
// from ../../gir-files
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GRANITE_SERVICES_CONTRACTOR_ERROR_SERVICE_NOT_AVAILABLE);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_COLOR_SCHEME_DARK);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_COLOR_SCHEME_LIGHT);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_COLOR_SCHEME_NO_PREFERENCE);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_PAGE_STATUS_TYPE_ERROR);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_PAGE_STATUS_TYPE_NONE);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_PAGE_STATUS_TYPE_OFFLINE);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_PAGE_STATUS_TYPE_SUCCESS);
    PRINT_CONSTANT((gint) GRANITE_SETTINGS_PAGE_STATUS_TYPE_WARNING);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_ACCENT);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_BACK_BUTTON);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_BADGE);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_CARD);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_CHECKERBOARD);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_CIRCULAR);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_COLOR_BUTTON);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_DEFAULT_DECORATION);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_DESTRUCTIVE_ACTION);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_DIALOG_CONTENT_AREA);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_DIM_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_ERROR);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_FLAT);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_H1_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_H2_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_H3_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_H4_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_KEYCAP);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_LINKED);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_MENU);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_MENUITEM);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_MESSAGE_DIALOG);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_MODE_SWITCH);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_OSD);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_ROUNDED);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_SMALL_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_SUGGESTED_ACTION);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_TEMPERATURE);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_TERMINAL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_TITLE_LABEL);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_VIEW);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_WARMTH);
    PRINT_CONSTANT(GRANITE_STYLE_CLASS_WARNING);
    PRINT_CONSTANT(GRANITE_TOOLTIP_SECONDARY_TEXT_MARKUP);
    PRINT_CONSTANT(GRANITE_TRANSITION_DURATION_CLOSE);
    PRINT_CONSTANT(GRANITE_TRANSITION_DURATION_IN_PLACE);
    PRINT_CONSTANT(GRANITE_TRANSITION_DURATION_OPEN);
    return 0;
}
