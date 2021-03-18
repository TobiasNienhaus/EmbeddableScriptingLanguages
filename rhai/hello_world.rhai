let x = #{
	test: 5,
	hello: 100,
	te: "Hello World",
	nested: #{
		one: 1,
		two: "two",
		three: '3'
	}
};
print("x.test " + x.test);
print("x.hello " + x.hello);
print("x.te " + x.te);
print("x[test] " + x["test"]);
print("x[hello] " + x["hello"]);
print("x[te] " + x["te"]);
print("x " + x);
print(type_of(x.nested.three));

fn print_stuff(msg) {
	print("Hello World!: " + msg);
	10.
}

print(sin(print_stuff(5)));

print(sin(2.0));