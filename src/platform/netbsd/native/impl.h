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
        uint64_t kvaddr;
        int32_t protocol;
        uint16_t port;
    };

    struct socket_file_t
    {
        uint64_t kvaddr;
        pid_t pid;
    };

    int lsock_tcp(struct socket_info_t **list, size_t *nentries);
    int lsock_tcp6(struct socket_info_t **list, size_t *nentries);
    int lsock_udp(struct socket_info_t **list, size_t *nentries);
    int lsock_udp6(struct socket_info_t **list, size_t *nentries);

    int lsock_files(struct socket_file_t **list, size_t *nentries);

    char *proc_name(pid_t pid);
    char *proc_path(pid_t pid);

#ifdef __cplusplus
}
#endif
