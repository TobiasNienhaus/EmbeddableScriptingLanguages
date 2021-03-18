use clap::{App, SubCommand};

fn main() {
    let app = App::new("Embedded Scripting Languages - gluon and rhai")
        .author("Tobias Nienhaus")
        .about("A comparison of gluon and rhai")
        .subcommand(SubCommand::with_name("gluon").about("Execute the gluon test"))
        .subcommand(SubCommand::with_name("rhai").about("Execute the rhai test"));
    let matches = app.get_matches();

    if let Some(_) = matches.subcommand_matches("gluon") {
        println!("Gluon isn't supported anymore");
    } else if let Some(_) = matches.subcommand_matches("rhai") {
        rhai_test();
    } else {
        assert!(false, "Not supported");
    }
}

fn rhai_test() {
    use rhai::*;
    let engine = Engine::new();
    let res = engine.eval::<i64>("40 + 2").unwrap();
    println!("Answer: {}", res);
}
