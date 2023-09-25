#include "helpwindow.hh"

const char *help_text = "<b>fnCalc</b>\nScripting Calculator\n\n"
                        "fnCalc is a miniature scripting language\n"
                        "suitable for calculations. You can define\n"
                        "and call functions, use variables and create\n"
                        "branches and loops.\n\n"
                        "<b>Using variables</b>\n\n"
                        "Simply assign a value to an identifier.\n"
                        "If a variable with the given identiefier\n"
                        "already exists, its value will be overwritten.\n\n"
                        "x = 42\n\n"
                        "You can then use the variable in other expressions.\n\n"
                        "y = x + 1\n\n"
                        "<b>Defining functions</b>\n\n"
                        "You can define functions using the 'fn' keyword.\n\n"
                        "fn power(base, exponent) {\n"
                        "    if (exponent == 0) {\n"
                        "      return 1\n"
                        "    }\n"
                        "    return base * power(base, exponent - 1)\n"
                        "}\n\n"
                        "<b>Built-in Functions:</b>\n\n"
                        "The following built-in functions are available:\n"
                        "sin, cos, tan, ln, log, abs\n\n"
                        "For trigonometric functions prepend 'a' for arcus\n"
                        "and append 'd' for degree.\n\n"
                        "Use 'pi' for an accurate value of the constant.\n\n"
                        "<b>Branches</b>\n\n"
                        "You can use the 'if' keyword with a control expression\n"
                        "for conditional execution. If the expression evaluates\n"
                        "to 0, the block will not run. Any other result than 0\n"
                        "will cause the block to be executed. Boolean operators\n"
                        "return 0 for 'false' and 1 for 'true'.\n\n"
                        "You can also use the 'else' keyword to specify a block\n"
                        "to be executed only in case the the control expression\n"
                        "evaluated to 'false'.\n\n"
                        "if x % 2 == 0 {\n"
                        "    print x\n"
                        "}\n"
                        "else {\n"
                        "    print x + 1\n"
                        "}\n\n"
                        "<b>Loops</b>\n\n"
                        "You can use the 'while' keyword with an expression for\n"
                        "conditional loops. The loop will continue to run as long\n"
                        "as the expression evaluates to a value other than 0.\n"
                        "Alternatively you can use the 'break' keyword to exit\n"
                        "the loop at an arbitrary point.\n\n"
                        "x = 0\n"
                        "while x &lt; 10 {\n"
                        "    x = x + 1\n"
                        "    if x == 7 {\n"
                        "        break\n"
                        "    }\n"
                        "}\n\n"
                        "<b>Printing</b>\n\n"
                        "You can use the 'print' keyword to print intermediary\n"
                        "values in in loops and function calls.\n\n"
                        "x = 0\n"
                        "while x &lt; 5 {\n"
                        "    print x * 2\n"
                        "    x = x + 1\n"
                        "}";

HelpWindow::HelpWindow(Gtk::Window &parent) {
    text.set_markup(help_text);
    text.set_margin(15);

    sc.set_policy(Gtk::PolicyType::NEVER, Gtk::PolicyType::AUTOMATIC);
    sc.set_child(text);

    set_child(sc);
    set_default_size(400, 300);
    set_title("Help");
    set_transient_for(parent);
    set_modal(true);
}

bool HelpWindow::on_close_request() {
    hide();
    return true;
}
