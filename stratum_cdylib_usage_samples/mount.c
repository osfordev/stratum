//
// gcc -g mount.c -o mount -lstratum -L../target/debug && LD_LIBRARY_PATH=../target/debug ./mount
//

#include "../stratum_cdylib/bindings_c.h"

int main() {
    stratum_mount();
}