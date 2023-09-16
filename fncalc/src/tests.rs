use crate::{process, reset_session};

#[test]
fn invalid_math() {
    let result = process("1 / 0");
    let expected = "1 / 0\n  ^\nError: Division by zero";
    assert_eq!(result, expected);

    let result = process("5/(10 - 2 * 5)");
    let expected = "5/(10 - 2 * 5)\n ^\nError: Division by zero";
    assert_eq!(result, expected);

    let result = process("tan(pi/2)");
    let expected = "tan(pi/2)\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("tan(5 * pi/2)");
    let expected = "tan(5 * pi/2)\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("tand(90)");
    let expected = "tand(90)\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("tand(5 * 90)");
    let expected = "tand(5 * 90)\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("tand(90 + 180 * 3)");
    let expected = "tand(90 + 180 * 3)\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("asin 2");
    let expected = "asin 2\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("acos -2");
    let expected = "acos -2\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("asind -2");
    let expected = "asind -2\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("acosd 2");
    let expected = "acosd 2\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("ln 0");
    let expected = "ln 0\n^\nError: Math error";
    assert_eq!(result, expected);

    let result = process("log -1");
    let expected = "log -1\n^\nError: Math error";
    assert_eq!(result, expected);
}

#[test]
fn expression() {
    let result = process("");
    let expected = "";
    assert_eq!(result, expected);

    let result = process("(1 / 3) * 3");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("5+(2.0-3)*4.9");
    let expected = "0.1";
    assert_eq!(result, expected);

    let result = process("5+2.0-3*4.9");
    let expected = "-7.7";
    assert_eq!(result, expected);

    let result = process("5 + -2");
    let expected = "3";
    assert_eq!(result, expected);

    let result = process("2 + -2 ^ 8");
    let expected = "258";
    assert_eq!(result, expected);

    let result = process("10/5");
    let expected = "2";
    assert_eq!(result, expected);

    let result = process("2 + -2 ^ 8");
    let expected = "258";
    assert_eq!(result, expected);

    let result = process("1 / 5");
    let expected = "0.2";
    assert_eq!(result, expected);

    let result = process("-(2+3)");
    let expected = "-5";
    assert_eq!(result, expected);

    let result = process("-(2+3) + 1");
    let expected = "-4";
    assert_eq!(result, expected);

    let result = process("-(2^3)");
    let expected = "-8";
    assert_eq!(result, expected);

    let result = process("-(2^3) + 1");
    let expected = "-7";
    assert_eq!(result, expected);

    let result = process("2^-(1)*3");
    let expected = "1.5";
    assert_eq!(result, expected);

    let result = process("-(3/6)");
    let expected = "-0.5";
    assert_eq!(result, expected);

    let result = process("-(3/6) + 1");
    let expected = "0.5";
    assert_eq!(result, expected);

    let result = process("456-41-675*2^3-15");
    let expected = "-5000";
    assert_eq!(result, expected);

    let result = process("(456-41-675)*2^3-15");
    let expected = "-2095";
    assert_eq!(result, expected);

    let result = process("3+4*50/5^2%5-1");
    let expected = "5";
    assert_eq!(result, expected);

    let result = process("(1+-4/2.5)*16-(7%2)^3/5");
    let expected = "-9.8";
    assert_eq!(result, expected);

    let result = process("((1+-4)/2.5)*16-(7%2)^3/5");
    let expected = "-19.4";
    assert_eq!(result, expected);

    let result = process("2^4*(10%4+17.5-5)/2.5");
    let expected = "92.8";
    assert_eq!(result, expected);

    let result = process("sin pi");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("cos (pi/2)");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("tan (pi/4)");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("asin 1");
    let expected = "1.570796";
    assert_eq!(result, expected);

    let result = process("acos 0");
    let expected = "1.570796";
    assert_eq!(result, expected);

    let result = process("atan pi");
    let expected = "1.262627";
    assert_eq!(result, expected);

    let result = process("sind 90");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("cosd 90");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("tand 45");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("asind 1");
    let expected = "90";
    assert_eq!(result, expected);

    let result = process("acosd 1");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("atand 1");
    let expected = "45";
    assert_eq!(result, expected);

    let result = process("ln 2");
    let expected = "0.693147";
    assert_eq!(result, expected);

    let result = process("log 100");
    let expected = "2";
    assert_eq!(result, expected);

    let result = process("sind90^2");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("sin cos 0");
    let expected = "0.841471";
    assert_eq!(result, expected);

    let result = process("abs -5.25");
    let expected = "5.25";
    assert_eq!(result, expected);

    let result = process("abs 5.25");
    let expected = "5.25";
    assert_eq!(result, expected);

    let result = process("2 and 2");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("2 and 0");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("0 and 1");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("0 and 0");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("2 or 2");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("2 or 0");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("0 or 1");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("0 or 0");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("not 0");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("not 3");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("1 < 2");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("2 < 1");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("1 > 2");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("2 > 1");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("1 == 1");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("1 == 2");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("2 == 1");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("1 != 1");
    let expected = "0";
    assert_eq!(result, expected);

    let result = process("1 != 2");
    let expected = "1";
    assert_eq!(result, expected);

    let result = process("2 != 1");
    let expected = "1";
    assert_eq!(result, expected);
}

#[test]
fn scripting() {
    // variables
    let result = process("x");
    let expected = "x\n^\nError: Identifier not found";
    assert_eq!(result, expected);

    let _result = process("x = 5");
    reset_session();
    let result = process("x");
    let expected = "x\n^\nError: Identifier not found";
    assert_eq!(result, expected);

    let result = process("x = 3");
    let expected = "3";
    assert_eq!(result, expected);

    let result = process("x = 3\nx + 1");
    let expected = "4";
    assert_eq!(result, expected);

    let result = process("x = y = 1\nx + y");
    let expected = "2";
    assert_eq!(result, expected);

    let result = process("x = 1\n x = x + 10");
    let expected = "11";
    assert_eq!(result, expected);

    let result = process("x = (5 + 2) ^ 3\nx + 1");
    let expected = "344";
    assert_eq!(result, expected);

    // while_loop
    let result = process("x = 0; while x < 5 { x = x + 1; print x; }");
    let expected = "1\n2\n3\n4\n5";
    assert_eq!(result, expected);

    let result = process("x = 0; while x < 5 { x = x + 1; print x; if x == 3 { break }}");
    let expected = "1\n2\n3";
    assert_eq!(result, expected);

    let result = process("
x = y = z = 0
while x < 12 {
  while y < 4 {
    y = y + 1
  }
  x = x + 1
  if x + y > 9 {
    z = z + 1
    if x == 8 {
      if 1 and 1 and not 0 {
        break
      }
    }
  }
}
x + y + z");
    let expected = "15";
    assert_eq!(result, expected);

    // branching
    let result = process("
x = 0
if not 0 > 0 {
  if 5 == 3 or not 1 { x = x + 1 }
  if 0 < 11 - (2 * 5) and 5 / 5 { x = x + 5 }
}
x");
    let expected = "5";
    assert_eq!(result, expected);

    let result = process("x = 3; if x > 10 {x = x + 10} else { x = x - 1 } x");
    let expected = "2";
    assert_eq!(result, expected);

    // functions
    let result = process("pow(2, 6)");
    let expected = "pow(2, 6)\n^\nError: Identifier not found";
    assert_eq!(result, expected);

    let result = process("
fn power(base, exponent) {
    if exponent == 0 {
        return 1
    }
    return base * power(base, exponent - 1)
}

print power(2, 8)");
    let expected = "256";
    assert_eq!(result, expected);

    let result = process("
fn count_up () {
    x = 0
    while 1 {
        if x == 5 {
            break
        }
        else {
            x = x + 1
            print x
        }
    }
    if x == 5 {
        return add_one(x)
    }
    x = x + 10
}

fn add_one(lhs) {
    lhs + 1
}

print add_one(count_up())");
    let expected = "1\n2\n3\n4\n5\n7";
    assert_eq!(result, expected);

    let result = process("fn none() {}\n5 + none()");
    let expected = "5 + none()\n    ^\nError: Function did not return a value";
    assert_eq!(result, expected);

    let result = process("fn f(x) {x}\nf()");
    let expected = "f()\n^\nError: Invalid number of arguments passed to function";
    assert_eq!(result, expected);

    let result = process("fn f(x) {x}\nf(1, 2)");
    let expected = "f(1, 2)\n^\nError: Invalid number of arguments passed to function";
    assert_eq!(result, expected);
}

#[test]
fn iteration_limit() {
    let result = process("while 1 {}");
    let expected = "Error: Maximum iteration count reached";
    assert_eq!(result, expected);

    let result = process("fn inf_rec() { inf_rec() }\ninf_rec()");
    let expected = "Error: Maximum iteration count reached";
    assert_eq!(result, expected);
}
