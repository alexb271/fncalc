use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::{env, fs, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: {} [file_path]", args[0]);
        std::process::exit(1);
    }
    if args.len() == 2 {
        if args[1] == "--help" {
            println!("{}", HELP_TEXT);
        } else {
            process_file(&args[1]);
        }
    } else {
        interpreter();
    }
}

fn process_file(file_path: &str) {
    let input = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let output = fncalc::process(&input);
    if !output.is_empty() {
        println!("{}", fncalc::process(&input));
    }
}

fn readline(rl: &mut DefaultEditor, prompt: &str) -> Result<String, ReadlineError> {
    let readline = rl.readline(prompt);
    match readline {
        Ok(line) => {
            _ = rl.add_history_entry(line.as_str());
            Ok(line)
        }
        Err(err) => Err(err),
    }
}

fn interpreter() {
    println!("[fnCalc v1.0]");
    let mut rl = DefaultEditor::new().expect("Failed to initialize rustyline");
    loop {
        let mut input = String::new();

        while input.is_empty() || input.ends_with('\\') {
            if input.ends_with('\\') {
                input.pop();
            }

            let mut line = readline(&mut rl, ">>> ").unwrap_or_else(|e| {
                eprintln!("{e}");
                std::process::exit(1);
            });

            if line.ends_with('\n') {
                line.pop();
            }

            input.push_str(&line);
        }

        match input.as_str() {
            "q" | "Q" | "exit" => {
                break;
            }
            "clear" => {
                print!("\x1Bc");
                std::io::stdout()
                    .flush()
                    .expect("Failed to flush standard output");
            }
            "reset" => {
                fncalc::reset_session();
            }
            _ => {
                let output = fncalc::process(&input);
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
        }
    }
}

const HELP_TEXT: &str = "fnCalc
Scripting Calculator

fnCalc is a miniature scripting language \
suitable for calculations. You can define \
and call functions, use variables and create \
branches and loops.

# Using variables

Simply assign a value to an identifier. \
If a variable with the given identiefier \
already exists, its value will be overwritten.

x = 42

You can then use the variable in other expressions.

y = x + 1

# Defining functions

You can define functions using the 'fn' keyword.

fn power(base, exponent) {
    if (exponent == 0) {
      return 1
    }
    return base * power(base, exponent - 1)
}

# Built-in Functions:

The following built-in functions are available:
sin, cos, tan, ln, log, abs

For trigonometric functions prepend 'a' for arcus \
and append 'd' for degree.

Use 'pi' for an accurate value of the constant.

# Branches

You can use the 'if' keyword with a control expression \
for conditional execution. If the expression evaluates \
to 0, the block will not run. Any other result than 0 \
will cause the block to be executed. Boolean operators \
return 0 for 'false' and 1 for 'true'.

You can also use the 'else' keyword to specify a block \
to be executed only in case the the control expression \
evaluated to 'false'.

if x % 2 == 0 {
    print x
}
else {
    print x + 1
}

# Loops

You can use the 'while' keyword with an expression for \
conditional loops. The loop will continue to run as long \
as the expression evaluates to a value other than 0. \
Alternatively you can use the 'break' keyword to exit \
the loop at an arbitrary point.

x = 0
while x &lt; 10 {
    x = x + 1
    if x == 7 {
        break
    }
}

# Printing

You can use the 'print' keyword to print intermediary \
values in in loops and function calls.

x = 0
while x &lt; 5 {
    print x * 2
    x = x + 1
}

# Miscellaneous

Supported operators: +, -, /, *, ^, %
You can use '\\' at the end of a line for multiline input
Input 'clear' to clear the screen
Input 'reset' to delete all functions and variables";
