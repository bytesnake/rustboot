arch=x86
# RUST_ROOT := /usr
RUST_ROOT := /home/piotr/
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/
SHELL := /bin/bash

-include ./config.mk

export RUST_ROOT
export LLVM_ROOT
export GCC_PREFIX

all:
	@$(MAKE) all -C arch/$(arch)/ SHELL=$(SHELL)

%:
	@$(MAKE) $* -C arch/$(arch)/ SHELL=$(SHELL)
