#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void stratum_build();

void stratum_inspect();

void stratum_mount();

void stratum_pull();

void stratum_umount();

} // extern "C"
