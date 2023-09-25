#include <gtkmm.h>
#include "glibmm/miscutils.h"
#include "mainwindow.hh"

int main(int argc, char **argv) {
    auto app = Gtk::Application::create("org.fncalc");
    Glib::set_application_name("fnCalc");
    return app->make_window_and_run<MainWindow>(argc, argv);
}
