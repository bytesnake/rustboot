LIBCDIR := ../uClibc/lib

RUSTC := $(RUST_ROOT)/bin/rustc
#RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -g

CC := $(LLVM_ROOT)/bin/clang
UCFLAGS := -O3 -flto $(CFLAGS) -nostdlib -static $(LIBCDIR)/crt1.o $(LIBCDIR)/crti.o
LIBS := -Wl,--start-group -lc -Wl,--end-group $(LIBCDIR)/crtn.o

-include ./config.mk

.PHONY: all run debug clean

all: hello.elf

%.bc: %.rs
	echo $(RUSTCFLAGS)
	$(RUSTC) $(RUSTCFLAGS) --emit=bc $^

%.elf: %.bc
	$(CC) $(UCFLAGS) -L$(LIBCDIR) -o $@ $^ $(LIBS)
	# $(OBJCOPY) $@ -S -g

run:

debug:

clean:
	@cat .gitignore | xargs -I{} find . -name {} | xargs rm -f
