use clap::{App, SubCommand};

fn main() {
    let app = App::new("Embedded Scripting Languages - gluon and rhai")
        .author("Tobias Nienhaus")
        .about("A comparison of gluon and rhai")
        .subcommand(SubCommand::with_name("dyon").about("Execute the gluon test"))
        .subcommand(SubCommand::with_name("rhai").about("Execute the rhai test"));
    let matches = app.get_matches();

    if let Some(_) = matches.subcommand_matches("dyon") {
        dyon_test();
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
    println!("Trying to run script...");
    let result: Dynamic = engine.eval_file("./rhai/hello_world.rhai".into()).unwrap();
    println!("Result: {}", result);
}

fn dyon_test() {
    use dyon::{error, run};
    error(run("dyon/test.dyon"));
}
