/**
 * Basic C sample which runs on Neotron OS.
 */

#include "neotron.h"
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

const NeotronApi *g_api;

static int main(void);

/*
 * Called by Neotron OS when the binary is 'run'.
 */
int app_entry(const NeotronApi *f) {
  g_api = f;
  return main();
}

/*
 * Our main function.
 *
 * Just prints a message and exits.
 */
static int main(void) {
  // allocate a buffer
  void *buffer = calloc(1024, 1);
  // write a string into it
  snprintf(buffer, 1023, "Hello, world!\n");
  // print the buffer
  printf(buffer);
  // free the buffer
  free(buffer);
  return 0;
}

// End of file
