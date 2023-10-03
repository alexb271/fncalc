# fnCalc
### Scripting Calculator

fnCalc is a miniature scripting language
suitable for calculations. You can define
and call functions, use variables and create
branches and loops.

## Building

To build fnCalc with the command-line interface, 
all you need is Cargo. In the project directory:

```
cargo build --release
```

To build fnCalc with the graphical interface, first
install the following dependencies: gtkmm4, meson, ninja.
For example, to install the dependencies along with the
needed compilers on Debian:

```
sudo apt install libgtkmm-4.0-dev meson ninja-build cargo g++
```

Then in the project directory:

```
meson setup build -Dbuildtype=release
cd build
ninja
```

### Usage

#### Using variables

Simply assign a value to an identifier.
If a variable with the given identiefier
already exists, its value will be overwritten.

```
x = 42
```

You can then use the variable in other expressions.

```
y = x + 1
```

#### Defining functions

You can define functions using the 'fn' keyword.

```
fn power(base, exponent) {
    if (exponent == 0) {
      return 1
    }
    return base * power(base, exponent - 1)
}
```

#### Built-in Functions:

The following built-in functions are available:
sin, cos, tan, ln, log, abs

For trigonometric functions prepend 'a' for arcus
and append 'd' for degree.

Use 'pi' for an accurate value of the constant.

#### Branches

You can use the 'if' keyword with a control expression
for conditional execution. If the expression evaluates
to 0, the block will not run. Any other result than 0
will cause the block to be executed. Boolean operators
return 0 for 'false' and 1 for 'true'.

You can also use the 'else' keyword to specify a block
to be executed only in case the the control expression
evaluated to 'false'.

```
if x % 2 == 0 {
    print x
}
else {
    print x + 1
}
```

#### Loops

You can use the 'while' keyword with an expression for
conditional loops. The loop will continue to run as long
as the expression evaluates to a value other than 0.
Alternatively you can use the 'break' keyword to exit
the loop at an arbitrary point.

```
x = 0
while x &lt; 10 {
    x = x + 1
    if x == 7 {
        break
    }
}
```

#### Printing

You can use the 'print' keyword to print intermediary
values in in loops and function calls.

```
x = 0
while x < 5 {
    print x * 2
    x = x + 1
}
```
