max_stack: max_stack.rs
	mkdir -p bin/
	mkdir -p bin-test/
	rustc max_stack.rs --out-dir=bin

test: max_stack.rs
	rustc --test max_stack.rs --out-dir=bin-test
	bin-test/max_stack
