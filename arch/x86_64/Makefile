BINUTILS_PREFIX := /home/piotr/x86_64-efi-pe/bin/
OVMF_ROOT := /usr/share/ovmf

TARGET := x86_64-intel-linux

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -g

LD := $(BINUTILS_PREFIX)x86_64-efi-pe-ld
LDFLAGS := --oformat pei-x86-64 -pie -e efi_start --subsystem 10 

GDB := $(BINUTILS_PREFIX)gdb
QEMU := qemu-system-x86_64

# error: internal compiler error: unexpected failure
# note: the compiler hit an unexpected failure path. this is a bug.
# note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
BDIR := ./efi
CORE_LIB := ../../rust-core/core/lib.rs

# TODO: fix dep-info target
MODS := $(wildcard *.rs) $(wildcard */*.rs) $(wildcard ../../kernel/*.rs) $(wildcard ../../kernel/*/*.rs) $(wildcard ../../kernel/*/*/*.rs)

-include ./config.mk
-include $(BDIR)/core.d

.PHONY: all clean run debug

all: $(BDIR)/boot.efi
	@wc -c $^

# Library rust-core
$(LCORE): $(CORE_LIB)
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/core.d $(CORE_LIB) --emit=obj,link --crate-type=lib,rlib --out-dir $(BDIR)

# Compile rustboot
$(BDIR)/main.o: ../../lib.rs $(MODS) $(LCORE)
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/main.d -L $(BDIR) --crate-type=lib --emit=obj ../../lib.rs --out-dir $(BDIR)

$(BDIR)/boot.efi: $(BDIR)/main.o
	$(LD) $(LDFLAGS) $^ -o $@ $(BDIR)/core.o

# running
run: all
	$(QEMU) -L $(OVMF_ROOT) -hda fat:boot/ -no-kvm -m 32

clean:
	@cat $(BDIR)/.gitignore | xargs -I{} find $(BDIR) -name {} | xargs rm -f

# debug: $(BDIR)/boot.efi
# ifeq ($(strip $(TMUX)),)
#         tmux new-session -d -s rustboot "$(QEMU) -L $(OVMF_ROOT) -hda fat:boot/ -no-kvm -m 32 -s -S"
#         tmux new-window -t rustboot:1 "$(GDB)"
#         tmux a -t rustboot
#         tmux kill-session -t rustboot
# else
#         tmux split-w "$(GDB); tmux kill-p"
#         $(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S
# endif
