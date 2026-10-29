use std::{cell::RefCell, io::{stdin, stdout, Write}, rc::Rc};

use crate::{environment::Environment, evaluator::evaluate, object::Object, parser::parse};

pub fn start(){
    let mut env = Environment::new();
    loop{
        let mut s=String::new();
        print!(">>");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let (program, errors) = parse(&s);

        if !errors.is_empty(){
            print_parser_error(errors);
            continue;
        }
        println!("{}",program.string());
        let func = Rc::new(RefCell::new(|str:&str|{
            println!("{str}")
        }));
        let evaluated = evaluate(program, &mut env, func);
        if evaluated != Object::Null{
           println!("{}",evaluated.inspect()); 
        }
        

    }
}
fn print_parser_error(errors:Vec<String>){
    println!("{}",MONKEY_FACE);
    println!("Woops! We ran into some monkey business here!");
    println!("parser errors:");

    for msg in errors{
        println!("{}",msg);
    }
}

const MONKEY_FACE: &str = r#"
          __,__
 .--.  .-"     "-.  .--.
/ .. \/ .-. .-.   \/ .. \
| | '| /   Y   \  |'  | |
| \ \ \  0 | 0 / /    / |
\ '- ,\.-"""""""-./, -' /
''-' /  _ ^ ^ _   \ '-''
    |  \._   _./   |
    \   \ '~' /   /
    '._  '-=-' _.'
       '-----'"#;