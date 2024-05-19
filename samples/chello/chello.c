/**
 * Basic C sample which runs on Neotron OS.
 */

#include "neotron.h"
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

const NeotronApi *g_api;

/*
 * Called by Neotron OS when the binary is 'run'.
 */
int app_entry(const NeotronApi *f, size_t argc, const FfiString* argv) {
  g_api = f;
  // allocate a buffer
  char *buffer = (char*) calloc(1024, 1);
  // write a string into it
  snprintf(buffer, 1023, "Hello, world!");
  // print the buffer
  printf("Buffer %p contains: '%s'\n", buffer, buffer);
  // free the buffer
  free(buffer);
  for(size_t i = 0; i < argc; i++) {
    const FfiString* ffi_arg = &argv[i];
    printf("Arg %u: %.*s\n", (unsigned int) i, ffi_arg->_0.data_len, ffi_arg->_0.data);   
  }
  return 0;
}

// End of file
