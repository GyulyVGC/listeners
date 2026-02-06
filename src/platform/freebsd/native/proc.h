#pragma once

#include "common.h"

#ifdef __cplusplus
extern "C"
{
#endif
    int proc_list(struct process_info_t **list, size_t *nentries);
    int proc_sockets(pid_t pid, struct socket_info_t **list, size_t *nentries);

#ifdef __cplusplus
}
#endif