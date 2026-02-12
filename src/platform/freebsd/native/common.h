#pragma once

#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/socketvar.h>
#include <sys/sysctl.h>
#include <sys/user.h> 
#include <netinet/in.h>
#include <netinet/in_pcb.h>
#include <netinet/tcp.h>
#include <netinet/tcp_var.h>
#include <netinet/udp.h>
#include <netinet/udp_var.h>
#include <arpa/inet.h>
#include <errno.h>
#include <libutil.h>
#include <unistd.h>

#ifdef __cplusplus
extern "C"
{
#endif

    enum protocol_t
    {
        PROTOCOL_TCP = 0,
        PROTOCOL_UDP = 1
    };

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
        uint16_t port;
        enum protocol_t protocol;
    };

    struct process_info_t {
        char  path[PATH_MAX];
        char  name[COMMLEN + 1];
        pid_t pid;
    };
#ifdef __cplusplus
}
#endif
