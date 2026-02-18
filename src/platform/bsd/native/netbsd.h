#pragma once

#ifdef __NetBSD__

#include "common.h"

#ifdef __cplusplus
extern "C"
{
#endif

    /**
     * netbsd_fetch_tcp_sockets - Retrieve all IPv4 TCP sockets on the system
     *
     * @list:     Output pointer that will point to a dynamically allocated array
     *            of socket_info_t structures. Caller is responsible for freeing it.
     * @nentries: Output number of TCP sockets in the list.
     *
     * Returns: 0 on success, -1 on failure (errno set accordingly).
     */
    int netbsd_fetch_tcp_sockets(struct socket_info_t **list, size_t *nentries);

    /**
     * netbsd_fetch_tcp6_sockets - Retrieve all IPv6 TCP sockets on the system
     *
     * @list:     Output pointer that will point to a dynamically allocated array
     *            of socket_info_t structures. Caller must free the array.
     * @nentries: Output number of IPv6 TCP sockets in the list.
     *
     * Returns: 0 on success, -1 on failure (errno set accordingly).
     */
    int netbsd_fetch_tcp6_sockets(struct socket_info_t **list, size_t *nentries);

    /**
     * netbsd_fetch_udp_sockets - Retrieve all IPv4 UDP sockets on the system
     *
     * @list:     Output pointer that will point to a dynamically allocated array
     *            of socket_info_t structures. Caller must free the array.
     * @nentries: Output number of IPv4 UDP sockets in the list.
     *
     * Returns: 0 on success, -1 on failure (errno set accordingly).
     */
    int netbsd_fetch_udp_sockets(struct socket_info_t **list, size_t *nentries);

    /**
     * netbsd_fetch_udp6_sockets - Retrieve all IPv6 UDP sockets on the system
     *
     * @list:     Output pointer that will point to a dynamically allocated array
     *            of socket_info_t structures. Caller must free the array.
     * @nentries: Output number of IPv6 UDP sockets in the list.
     *
     * Returns: 0 on success, -1 on failure (errno set accordingly).
     */
    int netbsd_fetch_udp6_sockets(struct socket_info_t **list, size_t *nentries);

    /**
     * netbsd_fetch_socket_files - Retrieve kernel socket file information
     *
     * @list:     Output pointer that will point to a dynamically allocated array
     *            of socket_file_t structures. Caller must free the array.
     * @nentries: Output number of socket files in the list.
     *
     * Returns: 0 on success, -1 on failure (errno set accordingly).
     */
    int netbsd_fetch_socket_files(struct socket_file_t **list, size_t *nentries);

    /**
     * netbsd_fetch_process_name - Retrieve the name of a process by PID
     *
     * @pid: Process ID.
     *
     * Returns: Dynamically allocated string containing the process name on success,
     *          or NULL on failure (errno set). Caller must free the string.
     */
    char *netbsd_fetch_process_name(pid_t pid);

    /**
     * netbsd_fetch_process_path - Retrieve the executable path of a process by PID
     *
     * @pid: Process ID.
     *
     * Returns: Dynamically allocated string containing the process path on success,
     *          or NULL on failure (errno set). Caller must free the string.
     */
    char *netbsd_fetch_process_path(pid_t pid);

#ifdef __cplusplus
}
#endif

#endif
