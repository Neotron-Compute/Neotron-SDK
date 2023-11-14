/*
 * Newlib stub functions we need.
 */

#include <stdint.h>
#include <sys/stat.h>
#include <unistd.h>

#include "neotron.h"

extern const NeotronApi *g_api;

/*
 * Implementation of the newlib library syscall `write`
 */
int _write(int fd, const void *data, size_t count) {
  if (fd >= 255) {
    return -1;
  }
  FfiByteSlice buffer = {
      .data = data,
      .data_len = count,
  };
  Handle neo_fd = {
      ._0 = (uint8_t)fd,
  };
  FfiResult_void result = g_api->write(neo_fd, buffer);
  if (result.tag == FfiResult_Ok) {
    return (int)count;
  } else {
    return -1;
  }
}

// End of file
