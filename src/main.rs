use clap::{App, SubCommand};

fn main() {
    let app = App::new("Embedded Scripting Languages - gluon and rhai")
        .author("Tobias Nienhaus")
        .about("A comparison of gluon and rhai")
        .subcommand(SubCommand::with_name("gluon").about("Execute the gluon test"))
        .subcommand(SubCommand::with_name("rhai").about("Execute the rhai test"));
    let matches = app.get_matches();

    if let Some(_) = matches.subcommand_matches("gluon") {
        gluon_test();
    } else if let Some(_) = matches.subcommand_matches("rhai") {
        rhai_test();
    }
    assert!(false, "Not supported");
}

fn gluon_test() {
    use gluon::*;
    use gluon::vm::api::IO;
    use gluon::import::*;
    // let mut vm = new_vm();
    // vm.run_io(true);
    // let script = r#"
    // let io = import! std.io
    // io.print "123"
    // "#;
    // // vm.run_expr::<IO<()>>(&vm, "test", script).unwrap();
    // // let (result, _): (api::IO<()>, _) = vm.run_expr("test_script", script).unwrap();
    // // let _: (api::IO<()>, _) = vm.run_expr("test_script", script).unwrap();
    // // vm.run_expr::<IO<()>>("example", script)
    // // .unwrap();
    // // Prints `123` to stdout
    // vm.run_io(true);
    // vm.run_expr::<IO<()>>("example", script)
    //     .unwrap();
    fn factorial(x: i32) -> i32 {
        if x <= 1 {
            1
        } else {
            x * factorial(x - 1)
        }
    }

    fn load_factorial(vm: &Thread) -> vm::Result<vm::ExternModule> {
        vm::ExternModule::new(vm, primitive!(1, factorial))
    }

    let vm = new_vm();

    // Introduce a module that can be loaded with `import! factorial`
    add_extern_module(&vm, "factorial", load_factorial);

    let expr = r#"
        let factorial = import! factorial
        factorial 5
    "#;
    println!("Before execution");
    let (result, _) = vm.run_expr::<i32>("factorial", expr)
        .unwrap();

    assert_eq!(result, 120);
}

fn rhai_test() {
    // use rhai::*;
    // let vm = Engine::new();
}
