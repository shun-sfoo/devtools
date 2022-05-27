.PHONY: gdb

# rust-gdb -quiet -pid $(shell pidof plaque) -ex "t a a bt" -ex "set confirm off" -ex "q" | 2>/dev/null | grep -A 90 'Thread 1' | grep -vE '[dD]etach(ing|ed)'

gdb:
	rust-gdb -quiet -pid $(shell pidof plaque) -ex "t a a bt" -ex "set confirm off" -ex "q" | grep -A 90 'Thread 1' | grep -vE '[dD]etach(ing|ed)'
