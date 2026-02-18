#pragma once

#include <sys/types.h>
#include <sys/sysctl.h>
#include <sys/file.h>
#include <stdlib.h>
#include <netinet/in.h>
#include <string.h>
#include <arpa/inet.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <sys/socket.h>
#include <sys/socketvar.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <netinet/udp.h>
#include <unistd.h>

#if defined(__FreeBSD__)
#include <netinet/in_pcb.h>
#include <netinet/tcp_var.h>
#include <netinet/udp_var.h>
#include <sys/user.h>
#endif

#if !defined(__FreeBSD__) && !defined(__OpenBSD__) && !defined(__NetBSD__)
    #error "Unsupported OS: please compile on FreeBSD, OpenBSD, or NetBSD"
#endif

#ifdef __cplusplus
extern "C"
{
#endif
    typedef struct
    {
        union
        {
            struct in_addr ipv4;
            struct in6_addr ipv6;
        } addr;

        int32_t family;
    } socket_address_t;

    struct socket_info_t
    {
        socket_address_t address;
#if defined(__FreeBSD__)
        kvaddr_t kvaddr;
#elif defined(__NetBSD__)
    uint64_t kvaddr;
#endif
        int32_t protocol;
        uint16_t port;
    };

#if defined(__FreeBSD__) || defined(__NetBSD__)
    struct socket_file_t
    {
#if defined(__FreeBSD__)
        kvaddr_t kvaddr;
#elif defined(__NetBSD__)
        uint64_t kvaddr;
#endif
        pid_t pid;
    };
#endif

#if defined(__OpenBSD__)
    struct proc_info_t
    {
        char name[KI_MAXCOMLEN];
        pid_t pid;
    };
#endif

    /**
     * sysctl_fetch - Retrieve sysctl data safely with retry protection
     *
     * This function wraps the BSD `sysctl()` call to safely allocate a buffer
     * for variable-length data, retrying in case the data size grows between
     * calls (TOCTOU protection). It attempts up to 3 retries if the buffer is
     * too small.
     *
     * @mib:        An array of integers representing the Management Information Base path.
     * @mnum:       Number of elements in the `mib` array.
     * @buffer:     Output pointer that will point to the allocated buffer containing sysctl data.
     *              The caller is responsible for freeing this buffer.
     * @buffer_size: Output size of the allocated buffer.
     *
     * Return:
     *   0          - Success, *buffer contains the sysctl data.
     *  -1          - Failure, errno is set to indicate the error:
     *                  EINVAL  - buffer or buffer_size is NULL
     *                  ENOMEM  - memory allocation failed
     *                  EOVERFLOW - data size changed too many times
     *                  Other  - sysctl returned an error
     */
    int sysctl_fetch(int mib[], int mnum, char **buffer, size_t *buffer_size);
#ifdef __cplusplus
}
#endif