ifeq ($(strip $(DEVKITPRO)),)
$(error "Please set DEVKITPRO in your environment. export DEVKITPRO=<path to>/devkitpro")
endif

all:
	aarch64-none-elf-gcc -o saltysd.o saltysd.c -c -I$(DEVKITPRO)/libnx/include
	aarch64-none-elf-ar rcs /workdir/lib/libsaltysd.a saltysd.o 

clean:
	rm saltysd.o libsaltysd.a
