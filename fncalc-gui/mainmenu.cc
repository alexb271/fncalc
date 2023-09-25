#include "mainmenu.hh"
#include "sigc++/functors/mem_fun.h"

MainMenu::MainMenu() {
    help_button.set_label("Help");
    help_button.set_has_frame(false);
    help_button.signal_clicked().connect(sigc::mem_fun(*this,
                &MainMenu::on_help_button_pressed));

    about_button.set_label("About");
    about_button.set_has_frame(false);
    about_button.signal_clicked().connect(sigc::mem_fun(*this,
                &MainMenu::on_about_button_pressed));

    box.set_orientation(Gtk::Orientation::VERTICAL);
    box.set_margin(10);
    box.append(help_button);
    box.append(about_button);

    set_child(box);
}

void MainMenu::on_help_button_pressed() {
    private_help_clicked.emit();
}

void MainMenu::on_about_button_pressed() {
    private_about_clicked.emit();
}

sigc::signal<void ()> MainMenu::signal_help_clicked() {
    return private_help_clicked;
}

sigc::signal<void ()> MainMenu::signal_about_clicked() {
    return private_about_clicked;
}
