/*
 * FreeRTOS Rust bindings 'C helper methods'.
 * Hopefully this won't be necessary for much longer,
 * but whatever Rust currently links with for libc,
 * it lacks some methods that you might expect to see.
 *
 * Like 'strlen'.
 *
 * I spent a little while trying to figure out how to
 * link a different libc, but it was hard to avoid
 * 'duplicate definition' errors with things that Rust
 * does include like 'memset'. And I couldn't get xargo
 * to work either. So...screw it, right? Just use shims
 * until Rust's bare metal toolchain is more finalized.
 */

#include <stddef.h>

// strlen: how many bytes is a null-terminated C string?
size_t strlen( const char * str ) {
  size_t l = 0;
  while ( str[ l ] != '\0' ) { ++l; }
  return l;
}
