#include "mainwindow.hh"
#include "fncalc.hh"
#include "calc_result.hh"

Glib::ustring insert_prompt_before_new_lines(Glib::ustring &input);
extern const char *style;

MainWindow::MainWindow()
: help_window(*this), key_controller(Gtk::EventControllerKey::create())
{
    add_controller(key_controller);
    key_controller->signal_key_pressed().connect(
                sigc::mem_fun(*this, &MainWindow::on_key_pressed), false);

    css_provider = Gtk::CssProvider::create();
    css_provider->load_from_data(style);
    Gtk::StyleContext::add_provider_for_display(Gdk::Display::get_default(), css_provider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);

    response_view.set_valign(Gtk::Align::FILL);
    response_view.set_expand(true);

    calculate_button.set_label("\u25B6"); // Unicode play button
    calculate_button.signal_clicked().connect(sigc::mem_fun(*this, &MainWindow::on_calculate_button_pressed));
    calculate_button.set_name("calculate_button");

    reset_button.set_icon_name("view-refresh-symbolic");
    reset_button.signal_clicked().connect(sigc::mem_fun(*this, &MainWindow::on_reset_button_pressed));

    main_menu_button.set_direction(Gtk::ArrowType::NONE);
    main_menu_button.set_popover(main_menu);
    main_menu.signal_about_clicked().connect(sigc::mem_fun(*this,
                &MainWindow::on_about_clicked));
    main_menu.signal_help_clicked().connect(sigc::mem_fun(*this,
                &MainWindow::on_help_clicked));

    input_field.set_hexpand(true);
    input_field.signal_activate().connect(sigc::mem_fun(*this, &MainWindow::on_calculate_button_pressed));

    entry_box.set_orientation(Gtk::Orientation::HORIZONTAL);
    entry_box.set_halign(Gtk::Align::FILL);
    entry_box.set_hexpand(true);
    entry_box.set_spacing(15);
    entry_box.append(input_field);
    entry_box.append(calculate_button);

    box.set_orientation(Gtk::Orientation::VERTICAL);
    box.set_margin(15);
    box.set_spacing(15);
    box.append(response_view);
    box.append(entry_box);

    header.set_show_title_buttons(true);
    header.pack_start(reset_button);
    header.pack_end(main_menu_button);

    set_child(box);
    set_titlebar(header);
    set_title("fnCalc");
    set_default_size(500, 450);
    set_default_widget(input_field);
    input_field.grab_focus();
}

void MainWindow::on_calculate_button_pressed() {
    Glib::ustring input = input_field.get_text();

    response_view.append_text(">>> ");
    response_view.append_text(insert_prompt_before_new_lines(input).c_str());

    if (!input.empty() && input[input.size() - 1] != '\n') {
        response_view.append_text("\n");
    }

    CalcResult output(input.c_str());
    response_view.append_text(output.str());
    response_view.append_text("\n");
    response_view.update_view_and_scroll_to_bottom();

    input_field.set_text("");
}

void MainWindow::on_reset_button_pressed() {
    fncalc_reset();
    response_view.clear();
}

void MainWindow::on_help_clicked() {
    main_menu.hide();
    help_window.show();
}

void MainWindow::on_about_clicked() {
    main_menu.hide();
    about_dialog = std::make_unique<Gtk::AboutDialog>();

    about_dialog->set_logo_icon_name("fncalc");
    about_dialog->set_program_name("fnCalc");
    about_dialog->set_comments("Scripting Calculator");
    about_dialog->set_version("1.0");
    about_dialog->set_license_type(Gtk::License::GPL_3_0);
    about_dialog->set_transient_for(*this);
    about_dialog->set_modal(true);
    about_dialog->show();
}

bool MainWindow::on_key_pressed(guint keyval, guint keycode, Gdk::ModifierType state) {
    // Ctrl + Q to exit
    if ((static_cast<int>(state) & GDK_CONTROL_MASK) && keyval == 'q') {
        hide();
        return true;
    }
    return false;
}

Glib::ustring insert_prompt_before_new_lines(Glib::ustring &input) {
    Glib::ustring output = input;

    size_t found = output.find('\n');
    while (found != std::string::npos) {
        output.replace(found, 1, "\n>>> ");
        found = output.find('\n', found + 5);
    }

    return output;
}
