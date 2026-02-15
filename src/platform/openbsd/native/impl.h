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
        int32_t protocol;
        uint16_t port;
    };

    struct proc_info_t
    {
        char name[KI_MAXCOMLEN];
        pid_t pid;
    };

    int proc_all(struct proc_info_t **list, size_t *nentries);
    int socks_by_pid(pid_t pid, struct socket_info_t **list, size_t *nentries);

#ifdef __cplusplus
}
#endif
