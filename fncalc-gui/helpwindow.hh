#pragma once

#include <gtkmm.h>

class HelpWindow : public Gtk::Window {
    private:
        Gtk::ScrolledWindow sc;
        Gtk::Label text;

        bool on_close_request() override;

    public:
        HelpWindow(Gtk::Window &parent);
};
