#![allow(non_snake_case)]
use library::{
    ast::Node, environment::Environment, evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser
};
use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use tracing::Level;
use wasm_bindgen::JsCast;

static CSS: Asset = asset!("/assets/main.css");
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    // Build cool things ✌️

    rsx! {

        document::Link { rel: "stylesheet", href: CSS }
        //document::Stylesheet { href: CSS }
        Hello{

        }
    }
}

pub fn test (input:&str) {


}
static helloworld:&str = "puts(\"hello world\")";

static builtins:&str =r#"let map ={
    true:[1,2,3,4,5,6],
    false:"hello world"
};

puts(map[true]);
puts(map[true][0]);
puts(map[true][10]);
puts(first(map[true]));
puts(last(map[true]));
puts(len(map[true]));
puts(len(map[false]));
let a =[1];
let b= push(a,2)
puts(b);
puts(rest(b));"#;

static factorial:&str = r#"let factorial = fn(n) {
  if (n <= 1) {
    return 1;
  }
  return n * factorial(n - 1);
}
puts(factorial(5))"#;

static fizzbuzz:&str=r#"let fizzbuzz = fn(n) {
    if (n == 1) {
        puts(1);
        return 0;
    }

    if (n % 15 == 0) {
        puts("fizzbuzz");
        return fizzbuzz(n-1);
    }
    if (n % 5 == 0) {
        puts("buzz");
        return fizzbuzz(n-1);
    }
    if (n % 3 == 0) {
        puts("fizz");
        return fizzbuzz(n-1);
    }

    puts(n);
    fizzbuzz(n-1);
};

fizzbuzz(100);"#;

static double_map:&str = r#"let map = fn(arr, f) {
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
puts("After double: ", map(a, double));"#;

pub fn Hello() -> Element {
    let mut user_input = use_signal::<String>(|| "puts(\"hello world\")".to_string());
    let mut output = use_signal::<Vec<String>>(|| vec![]);
    let mut caret = use_signal::<Option<(u32, u32)>>(|| None);
    use_effect(move || {
        let Some((start, end)) = caret() else {
            return;
        };
        if let Some(textarea) = code_input() {
            let _ = textarea.set_selection_range(start, end);
        }
    });
    let xx = move |str: &str| {
        output.push(str.to_owned());
    };
    let func = Rc::new(RefCell::new(xx));
    let source = user_input.read().clone();
    let source_lines = line_count(&source);
    let source_cols = column_count(&source);
    let source_height = block_height(source_lines);
    let printed = output.read().clone();
    let output_text = printed.join("\n");
    let output_lines = line_count(&output_text);
    let output_cols = column_count(&output_text);
    let output_height = block_height(output_lines);
    let tokens = highlight(&source);

    rsx! {
        div {
            class: "workspace",
            div {
                class: "pane input-section",
                div {
                    class: "tabs",
                    button {
                        class: tab_class(&source, helloworld),
                        r#type: "button",
                        onclick: move |_| user_input.set(helloworld.to_string()),
                        "Hello World"
                    },
                    button {
                        class: tab_class(&source, builtins),
                        r#type: "button",
                        onclick: move |_| user_input.set(builtins.to_string()),
                        "Built-in"
                    },
                    button {
                        class: tab_class(&source, factorial),
                        r#type: "button",
                        onclick: move |_| user_input.set(factorial.to_string()),
                        "Factorial"
                    },
                    button {
                        class: tab_class(&source, fizzbuzz),
                        r#type: "button",
                        onclick: move |_| user_input.set(fizzbuzz.to_string()),
                        "FizzBuzz"
                    },
                    button {
                        class: tab_class(&source, double_map),
                        r#type: "button",
                        onclick: move |_| user_input.set(double_map.to_string()),
                        "Double with Map"
                    },
                },
                div {
                    class: "editor",
                    div { class: "editor-title", "main.monkey" },
                    div {
                        class: "editor-scroll",
                        div {
                            class: "editor-grid",
                            style: "--code-height: {source_height}px; --code-cols: {source_cols}",
                            Gutter { count: source_lines },
                            div {
                                class: "code-layer",
                                pre {
                                    class: "highlight",
                                    for (i, tok) in tokens.iter().enumerate() {
                                        span {
                                            key: "{i}",
                                            class: "{tok.tone.class_name()}",
                                            "{tok.text}"
                                        }
                                    }
                                },
                                textarea {
                                    class: "code-input",
                                    id: "codeInput",
                                    spellcheck: "false",
                                    wrap: "off",
                                    value: "{source}",
                                    oninput: move |event| user_input.set(event.value()),
                                    onkeydown: move |event| {
                                        if event.key() != Key::Tab {
                                            return;
                                        }
                                        event.prevent_default();
                                        let Some(textarea) = event
                                            .downcast::<web_sys::KeyboardEvent>()
                                            .and_then(|raw| raw.target())
                                            .and_then(|target| {
                                                target.dyn_into::<web_sys::HtmlTextAreaElement>().ok()
                                            })
                                            .or_else(code_input)
                                        else {
                                            return;
                                        };
                                        let start = textarea.selection_start().ok().flatten().unwrap_or(0);
                                        let end = textarea.selection_end().ok().flatten().unwrap_or(start);
                                        let outdent = event.modifiers().contains(Modifiers::SHIFT);
                                        let (next, sel_start, sel_end) =
                                            apply_indent(&user_input.read(), start, end, outdent);
                                        user_input.set(next);
                                        caret.set(Some((sel_start, sel_end)));
                                    },
                                }
                            }
                        }
                    }
                },
                div {
                    class: "run-bar",
                    button {
                        class: "run-button",
                        r#type: "button",
                        onclick: move |_| {
                            let val = user_input.read().clone();
                            output.set(vec![]);
                            evaluate(val, func.clone());
                        },
                        "Run"
                    }
                }
            },
            div {
                class: "pane output-section",
                div { class: "tabs-spacer" },
                div {
                    class: "editor",
                    div { class: "editor-title", "output" },
                    div {
                        class: "editor-scroll",
                        id: "output",
                        div {
                            class: "editor-grid",
                            style: "--code-height: {output_height}px; --code-cols: {output_cols}",
                            Gutter { count: output_lines },
                            div {
                                class: "code-layer",
                                pre { class: "editor-text", "{output_text}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Gutter(count: usize) -> Element {
    rsx! {
        div {
            class: "gutter",
            for n in 1..=count {
                span { key: "{n}", "{n}" }
            }
        }
    }
}

fn tab_class(current: &str, sample: &str) -> &'static str {
    if current == sample { "tab active" } else { "tab" }
}

fn line_count(text: &str) -> usize {
    if text.is_empty() {
        1
    } else {
        text.split('\n').count()
    }
}

fn column_count(text: &str) -> usize {
    text.split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(1)
}

fn block_height(lines: usize) -> usize {
    lines.max(1) * 22 + 16
}

struct Token {
    tone: Tone,
    text: String,
}

#[derive(Clone, Copy)]
enum Tone {
    Plain,
    Keyword,
    Builtin,
    Ident,
    String,
    Number,
}

impl Tone {
    fn class_name(self) -> &'static str {
        match self {
            Tone::Plain => "",
            Tone::Keyword => "tok-kw",
            Tone::Builtin => "tok-builtin",
            Tone::Ident => "tok-ident",
            Tone::String => "tok-string",
            Tone::Number => "tok-number",
        }
    }
}

fn highlight(src: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            let mut text = String::from(c);
            while let Some(n) = chars.next() {
                text.push(n);
                if n == '\\' {
                    if let Some(escaped) = chars.next() {
                        text.push(escaped);
                    }
                } else if n == '"' {
                    break;
                }
            }
            out.push(Token { tone: Tone::String, text });
        } else if c.is_ascii_digit() {
            let mut text = String::from(c);
            while chars.peek().is_some_and(|n| n.is_ascii_digit()) {
                text.push(chars.next().unwrap());
            }
            out.push(Token { tone: Tone::Number, text });
        } else if c.is_ascii_alphabetic() || c == '_' {
            let mut text = String::from(c);
            while chars.peek().is_some_and(|n| n.is_ascii_alphanumeric() || *n == '_') {
                text.push(chars.next().unwrap());
            }
            let tone = match text.as_str() {
                "let" | "fn" | "if" | "else" | "return" | "true" | "false" => Tone::Keyword,
                "len" | "first" | "last" | "rest" | "push" | "puts" => Tone::Builtin,
                _ => Tone::Ident,
            };
            out.push(Token { tone, text });
        } else if c.is_whitespace() {
            let mut text = String::from(c);
            while chars.peek().is_some_and(|n| n.is_whitespace()) {
                text.push(chars.next().unwrap());
            }
            out.push(Token { tone: Tone::Plain, text });
        } else {
            out.push(Token { tone: Tone::Plain, text: c.to_string() });
        }
    }
    out
}

const INDENT: &str = "    ";

fn code_input() -> Option<web_sys::HtmlTextAreaElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id("codeInput")?
        .dyn_into()
        .ok()
}

fn apply_indent(source: &str, start_u16: u32, end_u16: u32, outdent: bool) -> (String, u32, u32) {
    let mut start = utf16_to_byte(source, start_u16 as usize);
    let mut end = utf16_to_byte(source, end_u16 as usize);
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    if !outdent && !source[start..end].contains('\n') {
        let mut next = String::with_capacity(source.len() + INDENT.len());
        next.push_str(&source[..start]);
        next.push_str(INDENT);
        next.push_str(&source[end..]);
        let cursor = byte_to_utf16(&next, start + INDENT.len()) as u32;
        return (next, cursor, cursor);
    }

    let block_start = source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let selection_end = if end > start && source.as_bytes().get(end - 1) == Some(&b'\n') {
        end - 1
    } else {
        end
    };
    let block_end = source[selection_end..]
        .find('\n')
        .map(|i| selection_end + i)
        .unwrap_or(source.len());
    let lines: Vec<&str> = source[block_start..block_end].split('\n').collect();

    let mut new_block = String::new();
    let mut new_start = start;
    let mut new_end = end;
    let mut old_at = block_start;
    let mut new_at = block_start;
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            new_block.push('\n');
            old_at += 1;
            new_at += 1;
        }
        let remove = if outdent {
            line.bytes().take_while(|byte| *byte == b' ').take(INDENT.len()).count()
        } else {
            0
        };
        let add = if outdent { 0 } else { INDENT.len() };
        if (block_start..block_end).contains(&start) || start == block_end {
            if (old_at..=old_at + line.len()).contains(&start) {
                let into = start - old_at;
                let into_new = if into <= remove { 0 } else { into - remove } + add;
                new_start = new_at + into_new;
            }
        }
        if (block_start..=block_end).contains(&end) && (old_at..=old_at + line.len()).contains(&end) {
            let into = end - old_at;
            let into_new = if into <= remove { 0 } else { into - remove } + add;
            new_end = new_at + into_new;
        }
        if outdent {
            new_block.push_str(&line[remove..]);
        } else {
            new_block.push_str(INDENT);
            new_block.push_str(line);
        }
        old_at += line.len();
        new_at += line.len() - remove + add;
    }

    let mut next = String::with_capacity(source.len() + new_block.len());
    next.push_str(&source[..block_start]);
    next.push_str(&new_block);
    next.push_str(&source[block_end..]);
    let sel_start = byte_to_utf16(&next, new_start.min(next.len())) as u32;
    let sel_end = byte_to_utf16(&next, new_end.min(next.len())) as u32;
    (next, sel_start, sel_end)
}

fn utf16_to_byte(text: &str, utf16_index: usize) -> usize {
    let mut seen = 0usize;
    for (byte, ch) in text.char_indices() {
        if seen >= utf16_index {
            return byte;
        }
        seen += ch.len_utf16();
    }
    text.len()
}

fn byte_to_utf16(text: &str, byte_index: usize) -> usize {
    text[..byte_index.min(text.len())].chars().map(|ch| ch.len_utf16()).sum()
}

fn evaluate<'a>(input: String, output:Rc<RefCell<dyn FnMut(&str) + 'a>>){

    let mut env = Environment::new();
    let l = Lexer::new(&input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    if p.errors().len() != 0 {
        print_parser_error(p.errors,output.clone());
    }

    let evaluator = Evaluator::new(output.clone());
    let evaludated = evaluator.eval(&Node::Program(program), &mut env);
    if evaludated != Object::Null {
         evaludated.inspect();
    }else{
        output.borrow_mut()("Null");
    }
}
fn print_parser_error<'a>(errors: Vec<String>,output:Rc<RefCell<dyn FnMut(&str) + 'a>>) {

    output.borrow_mut()("Woops! We ran into some monkey business here!");
    output.borrow_mut()("parser errors:");
    for msg in errors{
        output.borrow_mut()(&msg);
    }
}
