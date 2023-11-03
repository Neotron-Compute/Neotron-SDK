/**
 * Basic C sample which runs on Neotron OS.
 */

#include "neotron.h"

int app_entry(const NeotronApi *f)
{
    FfiByteSlice buffer = {
        .data = "Hello, world",
        .data_len = 12
    };
    Handle fd = {
        // fd 1 is STDOUT
        ._0 = 1
    };
    f->write(fd, buffer);
    return 0;
}
