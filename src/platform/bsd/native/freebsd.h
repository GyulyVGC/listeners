#pragma once
#ifdef __FreeBSD__

#include "common.h"

#ifdef __cplusplus
extern "C"
{
#endif

    /**
     * freebsd_fetch_tcp_sockets - Retrieve all TCP sockets in the system
     *
     * @list:      Output pointer that will be set to a dynamically allocated array
     *             of socket_info_t structures. Caller is responsible for freeing it.
     * @nentries:  Output number of entries in the returned list.
     *
     * Returns: 0 on success, -1 on failure (errno is set accordingly).
     */
    int freebsd_fetch_tcp_sockets(struct socket_info_t **list, size_t *nentries);

    /**
     * freebsd_fetch_udp_sockets - Retrieve all UDP sockets in the system
     *
     * @list:      Output pointer that will be set to a dynamically allocated array
     *             of socket_info_t structures. Caller is responsible for freeing it.
     * @nentries:  Output number of entries in the returned list.
     *
     * Returns: 0 on success, -1 on failure (errno is set accordingly).
     */
    int freebsd_fetch_udp_sockets(struct socket_info_t **list, size_t *nentries);

    /**
     * freebsd_fetch_socket_files - Retrieve kernel socket file information
     *
     * @list:      Output pointer that will be set to a dynamically allocated array
     *             of socket_file_t structures. Caller is responsible for freeing it.
     * @nentries:  Output number of entries in the returned list.
     *
     * Returns: 0 on success, -1 on failure (errno is set accordingly).
     */
    int freebsd_fetch_socket_files(struct socket_file_t **list, size_t *nentries);

    /**
     * freebsd_fetch_process_name - Retrieve the name of a process given its PID
     *
     * @pid:       Process ID.
     *
     * Returns: Dynamically allocated string containing the process name on success,
     *          or NULL on failure (errno is set accordingly). Caller must free the string.
     */
    char *freebsd_fetch_process_name(pid_t pid);

    /**
     * freebsd_fetch_process_path - Retrieve the executable path of a process given its PID
     *
     * @pid:       Process ID.
     *
     * Returns: Dynamically allocated string containing the process path on success,
     *          or NULL on failure (errno is set accordingly). Caller must free the string.
     */
    char *freebsd_fetch_process_path(pid_t pid);

#ifdef __cplusplus
}
#endif

#endif