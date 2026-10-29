use std::{cell::RefCell, rc::Rc};

use crate::{evaluator::new_error, object::Object};

pub fn is_builtin(name: &str) -> bool {
    matches!(name, "len" | "first" | "last" | "rest" | "push" | "puts")
}

fn expect_array<'a>(
    args: &'a [Object],
    want_len: usize,
    count_label: usize,
    type_name: &str,
) -> Result<&'a [Object], Object> {
    if args.len() != want_len {
        return Err(new_error(format!(
            "wrong number of arguments. got={}, want={count_label}",
            args.len()
        )));
    }
    match &args[0] {
        Object::Array(items) => Ok(items),
        other => Err(new_error(format!(
            "argument to '{type_name}' must be ARRAY, got={}",
            other.get_type()
        ))),
    }
}


pub struct Builtins<'a>{
    output:Rc<RefCell<dyn FnMut(&str) + 'a>>
}
impl<'a> Builtins<'a>{
    pub fn new(output:Rc<RefCell<dyn FnMut(&str) + 'a>>)->Builtins<'a>{
        let builtins =Builtins{
            output
        };

        builtins
    }
    pub fn get(&self, fn_name: &str) -> Object {
        if is_builtin(fn_name) {
            Object::Builtin(fn_name.to_string())
        } else {
            Object::Null
        }
    }
    pub fn call(&self, name:&str, args:Vec<Object>)->Object{
        match name{
            "len" => self.len(args),
            "first" => self.first(args),
            "last" => self.last(args),
            "rest" => self.rest(args),
            "push" => self.push(args),
            "puts" => self.puts(args),
            _ => Object::Null
        }
    }

    fn len(&self,args:Vec<Object>)->Object{
        if args.len() !=1{
            return new_error(format!("wrong number of arguments. got={}, want=1",args.len()));
        }
        match &args[..]{
            [Object::String(v)] => Object::Integer(v.len() as i64),
            [Object::Array(a)] => Object::Integer(a.len() as i64),
            _=>new_error(format!("argument to 'len' not supported, got={:?}",args[0]))
        }
    }
    fn first(&self, args: Vec<Object>) -> Object {
        match expect_array(&args, 1, 1, "first") {
            Ok(items) => items[0].clone(),
            Err(error) => error,
        }
    }
    fn last(&self, args: Vec<Object>) -> Object {
        match expect_array(&args, 1, 1, "last") {
            Ok(items) if !items.is_empty() => items[items.len() - 1].clone(),
            Ok(_) => Object::Null,
            Err(error) => error,
        }
    }
    fn rest(&self, args: Vec<Object>) -> Object {
        match expect_array(&args, 1, 1, "rest") {
            Ok(items) if !items.is_empty() => Object::Array(items[1..].to_vec().into()),
            Ok(_) => Object::Null,
            Err(error) => error,
        }
    }
    fn push(&self, args: Vec<Object>) -> Object {
        match expect_array(&args, 2, 1, "rest") {
            Ok(items) => {
                let mut copied = items.to_vec();
                copied.push(args[1].clone());
                Object::Array(copied.into())
            }
            Err(error) => error,
        }
    }
    pub fn puts(&self, args:Vec<Object>)->Object{
        if args.len()==0{
            return Object::Null
        }
        let ret=args.iter().map(|x|x.inspect()).collect::<Vec<String>>().join(" ");

        self.output.borrow_mut()(&ret);
        Object::String(ret)
    }
}