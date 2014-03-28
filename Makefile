test: testdir max_stack_test

bindir:
	@mkdir -p bin/

testdir:
	@mkdir -p bin-test/

max_stack: bindir max_stack.rs
	rustc max_stack.rs --out-dir=bin

max_stack_test: max_stack.rs
	rustc --test max_stack.rs --out-dir=bin-test
	bin-test/max_stack
