#pragma once

#ifdef __OpenBSD__

#include "common.h"

#ifdef __cplusplus
extern "C"
{
#endif
    struct proc_info_t
    {
        char name[KI_MAXCOMLEN];
        pid_t pid;
    };

    /**
     * openbsd_fetch_processes - Retrieve all running processes
     *
     * @list:     Output pointer to an array of proc_info_t structures.
     *            The caller is responsible for freeing this memory.
     * @nentries: Output number of processes in the list.
     *
     * This function queries the kernel using sysctl to get all processes,
     * copies their PIDs and names into proc_info_t structures.
     *
     * Returns: 0 on success, -1 on failure (errno is set accordingly):
     *            - ENOMEM if memory allocation fails
     *            - other errno set by sysctl_fetch
     */
    int openbsd_fetch_processes(struct proc_info_t **list, size_t *nentries);

    /**
     * openbsd_fetch_sockets_by_pid - Retrieve TCP/UDP sockets owned by a process
     *
     * @pid:      Process ID whose sockets are to be fetched.
     * @list:     Output pointer to an array of socket_info_t structures.
     *            The caller is responsible for freeing this memory.
     * @nentries: Output number of sockets in the list.
     *
     * This function queries the kernel for all file descriptors of the
     * given process, counts TCP/UDP sockets, allocates an array, and
     * fills socket_info_t structures with protocol, port, and IP address info.
     *
     * Returns: 0 on success, -1 on failure (errno is set accordingly):
     *            - ENOMEM if memory allocation fails
     *            - other errno set by sysctl_call
     */
    int openbsd_fetch_sockets_by_pid(pid_t pid, struct socket_info_t **list, size_t *nentries);

#ifdef __cplusplus
}
#endif

#endif