#pragma once

#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/socketvar.h>
#include <sys/sysctl.h>
#include <sys/user.h> 
#include <sys/file.h>
#include <netinet/in.h>
#include <netinet/in_pcb.h>
#include <netinet/tcp.h>
#include <netinet/tcp_var.h>
#include <netinet/udp.h>
#include <netinet/udp_var.h>
#include <arpa/inet.h>
#include <errno.h>
#include <unistd.h>

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
        kvaddr_t kvaddr;
        int32_t protocol;
        uint16_t port;
    };

    struct socket_file_t
    {
        kvaddr_t kvaddr;
        pid_t pid;
    };

    int lsock_tcp(struct socket_info_t **list, size_t *nentries);
    int lsock_udp(struct socket_info_t **list, size_t *nentries);
    int lsock_files(struct socket_file_t** list, size_t *nentries);

#ifdef __cplusplus
}
#endif
