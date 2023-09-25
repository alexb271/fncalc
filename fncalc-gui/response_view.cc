#include "response_view.hh"

ResponseView::ResponseView() {
    text_buffer = Gtk::TextBuffer::create();

    text_view.set_buffer(text_buffer);
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.set_name("response_view");

    set_child(text_view);
    set_size_request(300, 200);
}

void ResponseView::append_text(const char *text) {
    text_buffer->insert(text_buffer->end(), text);
}

void ResponseView::update_view_and_scroll_to_bottom() {
    Glib::RefPtr<Glib::MainContext> context = Glib::MainContext::get_default();
    while (context->pending()) {
        context->iteration(false);
    }

    Glib::RefPtr<Gtk::Adjustment> v_adj = get_vadjustment();
    v_adj->set_value(v_adj->get_upper());
}

void ResponseView::clear() {
    text_view.get_buffer()->set_text("");
}
