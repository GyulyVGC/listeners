#include "common.h"

int sysctl_fetch(int mib[], int mnum, char **buffer, size_t *buffer_size)
{
    size_t retries = 3;

    if (!buffer || !buffer_size)
    {
        errno = EINVAL;
        return -1;
    }

    for (size_t i = 0; i < retries; i++)
    {
        size_t size = 0;
        if (sysctl(mib, mnum, NULL, &size, NULL, 0) < 0)
            return -1;

        *buffer = malloc(size);
        if (!*buffer)
        {
            errno = ENOMEM;
            return -1;
        }

        if (sysctl(mib, mnum, *buffer, &size, NULL, 0) == 0)
        {
            *buffer_size = size;
            return 0;
        }
        else if (errno == ENOMEM)
        {
            free(*buffer);
            continue;
        }
        else
        {
            free(*buffer);
            return -1;
        }
    }

    errno = EOVERFLOW;
    return -1;
}