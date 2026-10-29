
<p align="center">
  <a href="https://emptyfridge0900.github.io/interpreter/">
    <picture>
      <img src="https://monkeylang.org/images/logo.png" height="128">
    </picture>
    <h1 align="center">Monkey lang</h1>
  </a>
</p>

# About the project

This project is a rewrite of the code from [Writing An Interpreter In Go](https://interpreterbook.com/) in Rust. \
Writing an interpreter is fun and challenging, at the same time I thought it would be a good project to practice Rust.
And many people have already implemented monkeylang in other languages. And I got a lot of help from their code. https://monkeylang.org/


# How it is put together

The root `Cargo.toml` is a workspace. `library` has no dependencies. `console` and `web` both call it.

| Crate | Role |
| --- | --- |
| `library` | Lexer, parser, evaluator, builtins, and the REPL loop |
| `console` | Binary. `main` calls `library::repl::start` |
| `web` | Dioxus 0.7 playground. Same library, compiled to WASM |

## Pipeline

Both hosts run the same three steps.

1. `Lexer` reads the source as bytes and yields `Token`s. Whitespace is skipped. Keywords are `fn`, `let`, `true`, `false`, `if`, `else`, and `return`. Any other name is an identifier. Builtins are ordinary identifiers, resolved later by the evaluator.
2. `Parser` is a Pratt parser. Prefix and infix function maps turn the token stream into a `Program`, a list of statements. Statements are `let`, `return`, an expression, or a block. Expressions cover identifiers, integers, booleans, strings, prefix and infix operators, `if`, function literals, calls, arrays, index expressions, and hashes.
3. `Evaluator` walks that tree with an `Environment`. `let` stores a name. A function literal stores its parameters, body, and the environment where it was created. A call builds an enclosed environment, binds the arguments, and unwraps a `return`. A bare identifier is looked up in the environment, then in the builtins. The walk produces an `Object`.

`Object` is the runtime value: integer, boolean, null, string, array, hash, function, builtin, return, or error. `inspect` is the text the hosts show. An array renders as `[1, 2, 3]`. A string renders without extra quotes. Null renders as `null`.

Monkey has no `while` or `for`. The playground samples Factorial and FizzBuzz repeat by calling themselves. An array index outside the array is null. A hash lookup that misses the key is an error, `unusable as hash key`. Hash pair order in `inspect` follows the map, so the examples below do not print a whole hash.

## Hosts

### Console

From the repository root:

```bash
cargo run -p console
```

The prompt is `>>`. Each turn reads one line. Bindings survive the next line. A program that needs several lines belongs in the web editor.

Before evaluation the REPL prints the AST text from `program.string()`. Keyword tokens print in capitals, so a let prints as `LET a = [1, 2, 3, 4];`. String literals in that AST text have no quotes. After evaluation it prints `inspect`, and it skips a null result.

`puts` prints during evaluation. The REPL then prints the returned string again. A parser error prints the monkey face and skips evaluation for that line. Ctrl-C stops the process. Ctrl-D leaves the prompt running.

### Web

The playground is at <https://emptyfridge0900.github.io/interpreter/>. Locally, from `web/`:

```bash
dx serve --hot-reload
```

Run evaluates the whole editor buffer. `puts` lines are buffered and drawn together when the click handler returns, still in call order. The AST is not printed. The final value is not printed either, except a null result, which writes the word `Null`. Parser errors are written to the output pane, and evaluation still runs.

The tabs are Hello World, Built-in, Factorial, FizzBuzz, and Double with Map.

A push to `v2` builds `web` with `dx build --release --base-path /interpreter` and deploys `gh-pages`.

# Overview
## Programs 
Programs in Monkey are a series of statements.

In monkey language expressions produce values, statements don’t.

## Statements
### let statement
let \<identifier\> = \<expression\>;

    A let statement in Monkey consists of two changing parts: an identifier and an expression.
    Exampe:
        let foobar = 5
### return statement
return \<expression\>;

    Example:
        return add(x+y);

### expression statement
it’s a statement that consists solely of one expression

    Example:
        x + 10;

### block statement
 Block statements are a series of statements enclosed by an opening { and a closing }.

    Example:
        { true }

## Expressions 
### identifier

    foo * bar / foobar
    foo and bar  are identifier
    
### integer literal
\<number\>

    Example:
        5
    Another example integer literal as the parameters of a function 
        add(5, 10);
### boolean literal
\<boolean\>

    boolan literal as the expression in a let statement
    let foobar = true;
### prefix
-, !

    Examples:
        -5
        !true
### infix
+, -, /, %, *, ==, !=, <=, >=

    Example:
        5 + 5
### if
if (\<condition\>) \<consequence\> else \<alternative\>

    Example:
        if (10 > 5) { true } else { false };
### function literal
fn \<parameters\> \<block statement\>

    For example, here is a function literal as the expression in a let statement:
        ``let add = fn(x, y) { return x + y };``
    And here is a function literal as the expression in a return statement inside another function literal:
        fn() {
            return fn(x, y) { return x > y; };
        }
    Using a function literal as an argument when calling another function is also possible:
        myFunc(x, y, fn(x, y) { return x > y; });
### call
    add(2, 3)
### string literal
\<sequence of characters\>

    "any string"
### array literal
[comma seperated list of expressions]

    Example:
    [1,2,"3",add(4,5)]
### hash literal
{comma seperated list of key-value pairs}

    Example:
    {"name": "Jimmy", "age": 72, "band": "Led Zeppelin"};



## Built-in functions

Six functions: `len`, `first`, `last`, `rest`, `push`, and `puts`. A name resolves to a builtin only when the environment has no binding of that name. A `let` with the same name hides the builtin.

On the web, wrap a value in `puts` to see it. That host does not print the final value. The comments below are the returned value. Multi-line samples are for the web editor. The console reads one line at a time.

### len

`len(value)` takes one argument.

- A string returns its byte length. `len("hello")` is `5`.
- An array returns its element count. `len([1, 2, 3, 4])` is `4`.
- Any other type, or any other argument count, is an error object.

### first

`first(array)` takes one array and returns the element at index 0. Use a non-empty array. An empty array is not handled.

```
first([1, 2, 3, 4])    // 1
```

### last

`last(array)` takes one array. It returns the last element, or null when the array is empty.

```
last([1, 2, 3, 4])     // 4
last([])               // null
```

### rest

`rest(array)` takes one array and returns a new array without the first element. The input array stays as it was. An empty array returns null.

```
rest([1, 2, 3, 4])     // [2, 3, 4]
rest([])               // null
```

### push

`push(array, value)` takes an array and one value. It returns a new array with the value appended. The input array stays as it was.

```
let a = [1, 2, 3, 4];
push(a, 5)             // [1, 2, 3, 4, 5]
a                      // [1, 2, 3, 4]
```

### puts

`puts(...)` prints and returns the printed text. In *Writing An Interpreter In Go*, `puts` returns null. Here it returns the string it printed.

- No arguments: the result is null, and nothing is printed.
- One or more arguments: join `inspect` of each argument with a single space, pass that string to the host, and return the same string.

On the console, with `a` already bound to `[1, 2, 3, 4]`:

```
>> puts(first(a), last(a), len(a))
puts(first(a), last(a), len(a))
1 4 4
1 4 4
```

The first `1 4 4` is the print during evaluation. The second is the REPL printing the returned string. On the web the same call shows `1 4 4` once.

This program uses boolean hash keys:

```
let map = {
    true: [1, 2, 3, 4, 5, 6],
    false: "hello world"
};

puts(map[true]);
puts(first(map[true]));
puts(last(map[true]));
puts(len(map[true]));
puts(len(map[false]));
let a = [1];
let b = push(a, 2);
puts(b);
puts(rest(b));
```

The output pane shows these seven lines. The returned value of the last `puts` is not printed again.

```
[1, 2, 3, 4, 5, 6]
1
6
6
11
[1, 2]
[2]
```

`len(map[false])` is 11 because `"hello world"` is 11 bytes.

`len`, `first`, `rest`, and `push` can walk an array together. This is the Double with Map sample. `puts` inserts one space between arguments, and the string already ends with a space, so each line has two spaces before the array.

```
let map = fn(arr, f) {
    let iter = fn (arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(first(arr))));
        }
    };
    iter(arr, []);
};

let a = [1, 2, 3, 4];
let double = fn(x) { x * 2 };

puts("Before double: ", a);
puts("After double: ", map(a, double));
```

```
Before double:  [1, 2, 3, 4]
After double:  [2, 4, 6, 8]
```