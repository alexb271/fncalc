#include "input_field.hh"

InputField::InputField()
: key_controller(Gtk::EventControllerKey::create())
{
    text_buffer = Gtk::TextBuffer::create();
    text_view.set_buffer(text_buffer);
    text_view.set_monospace(true);
    text_view.set_name("input_field");
    text_view.add_controller(key_controller);
    key_controller->signal_key_pressed().connect(
                sigc::mem_fun(*this, &InputField::on_key_pressed), false);

    set_size_request(-1, 100);
    set_child(text_view);
}

void InputField::grab_focus() {
    text_view.grab_focus();
}

Glib::ustring InputField::get_text() {
    return text_buffer->get_text();
}

void InputField::set_text(Glib::ustring &text) {
    text_buffer->set_text(text);
}

void InputField::set_text(const char *text) {
    text_buffer->set_text(text);
}

sigc::signal<void ()> InputField::signal_activate() {
    return private_activate;
}

bool InputField::on_key_pressed(guint keyval, guint keycode, Gdk::ModifierType state) {
    // Ctrl + Enter
    if ((static_cast<int>(state) & GDK_CONTROL_MASK) && (keyval == GDK_KEY_Return || keyval == GDK_KEY_KP_Enter)) {
        private_activate.emit();
        return true;
    }
    return false;
}
