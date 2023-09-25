#pragma once

#include <gtkmm.h>

#include "mainmenu.hh"
#include "response_view.hh"
#include "input_field.hh"
#include "helpwindow.hh"

class MainWindow : public Gtk::Window
{
    private:
        Gtk::Box box;
        Gtk::Box entry_box;
        Gtk::Button calculate_button;
        Gtk::Button reset_button;
        Gtk::MenuButton main_menu_button;
        Gtk::HeaderBar header;

        MainMenu main_menu;
        ResponseView response_view;
        InputField input_field;
        HelpWindow help_window;

        Glib::RefPtr<Gtk::CssProvider> css_provider;
        Glib::RefPtr<Gtk::EventControllerKey> key_controller;
        std::unique_ptr<Gtk::AboutDialog> about_dialog;

        void on_calculate_button_pressed();
        void on_reset_button_pressed();
        void on_help_clicked();
        void on_about_clicked();
        bool on_key_pressed(guint keyval, guint keycode, Gdk::ModifierType state);

    public:
      MainWindow();
};
