#pragma once

#include <gtkmm.h>

class MainMenu : public Gtk::Popover {
    private:
        Gtk::Box box;
        Gtk::Button help_button;
        Gtk::Button about_button;

        sigc::signal<void ()> private_help_clicked;
        sigc::signal<void ()> private_about_clicked;

        void on_help_button_pressed();
        void on_about_button_pressed();

    public:
        MainMenu();
        sigc::signal<void ()> signal_help_clicked();
        sigc::signal<void ()> signal_about_clicked();
};
