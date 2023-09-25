#pragma once
#include <gtkmm.h>

class InputField : public Gtk::ScrolledWindow {
    private:
        Gtk::TextView text_view;
        Glib::RefPtr<Gtk::TextBuffer> text_buffer;
        Glib::RefPtr<Gtk::EventControllerKey> key_controller;

        sigc::signal<void ()> private_activate;

        bool on_key_pressed(guint keyval, guint keycode, Gdk::ModifierType state);

    public:
        InputField();
        void grab_focus();
        Glib::ustring get_text();
        void set_text(Glib::ustring &text);
        void set_text(const char *text);
        sigc::signal<void ()> signal_activate();
};
