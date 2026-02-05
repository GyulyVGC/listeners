#pragma once

#include "common.h"

#ifdef __cplusplus
extern "C"
{
#endif
    int lsock_tcp(struct socket_info_t **list, size_t *nentries);

    int lsock_tcp6(struct socket_info_t **list, size_t *nentries);

    int lsock_udp(struct socket_info_t **list, size_t *nentries);

    int lsock_udp6(struct socket_info_t **list, size_t *nentries);

#ifdef __cplusplus
}
#endif