.PHONY: prepare
prepare:
	cp answer/file.rs file/src/ans.rs
	cp answer/kernel_callback.rs kernel_callback/src/ans.rs
	cp answer/guard.rs guard/src/ans.rs

.PHONY: check_file
check_file: prepare
	cd file && ./eval.sh

.PHONY: check_kernel_callback
check_kernel_callback: prepare
	cd kernel_callback && ./eval.sh

.PHONY: check_guard
check_guard: prepare
	cd guard && ./eval.sh

.PHONY: check_all
check_all: prepare check_kernel_callback check_file check_guard
