#pragma once
#include <gtkmm.h>

class ResponseView : public Gtk::ScrolledWindow {
    private:
        Gtk::TextView text_view;
        Glib::RefPtr<Gtk::TextBuffer> text_buffer;

    public:
        ResponseView();
        void append_text(const char *text);
        void update_view_and_scroll_to_bottom();
        void clear();
};
