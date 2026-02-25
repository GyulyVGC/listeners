#include "netbsd.h"

#ifdef __NetBSD__

/**
 * count_sockets - Count TCP/UDP sockets in a kinfo_pcb array
 *
 * @sockets: Pointer to an array of kinfo_pcb structures returned by sysctl.
 * @socknum: Number of entries in the sockets array.
 *
 * Iterates over the array and counts only entries where the protocol
 * is TCP (IPPROTO_TCP) or UDP (IPPROTO_UDP).
 *
 * Returns: The number of TCP/UDP sockets found.
 */
static size_t count_sockets(struct kinfo_pcb *sockets, size_t socknum)
{
    size_t retval = 0;

    for (size_t i = 0; i < socknum; ++i)
    {
        if (sockets[i].ki_protocol != IPPROTO_TCP && sockets[i].ki_protocol != IPPROTO_UDP)
            continue;

        ++retval;
    }

    return retval;
}

/**
 * count_files - Count socket file entries in a kinfo_file array
 *
 * @files:    Pointer to an array of kinfo_file structures returned by sysctl.
 * @filesnum: Number of entries in the files array.
 *
 * Iterates over the array and counts only entries where the file type
 * is a socket (DTYPE_SOCKET).
 *
 * Returns: The number of socket file entries found.
 */
static size_t count_files(struct kinfo_file *files, size_t filesnum)
{
    size_t retval = 0;

    for (size_t i = 0; i < filesnum; ++i)
    {
        if (files[i].ki_ftype != DTYPE_SOCKET)
            continue;

        ++retval;
    }

    return retval;
}

/**
 * fetch_sockets_common - Fetch TCP/UDP sockets and populate socket_info_t array
 *
 * @list:     Output pointer to an array of socket_info_t structures. Caller
 *            is responsible for freeing the allocated memory.
 * @nentries: Output number of entries in the array.
 * @mib:      Array describing the sysctl MIB to fetch the socket PCB data.
 * @mib_size: Number of elements in the MIB array.
 *
 * This function queries the kernel using sysctl, counts TCP/UDP sockets,
 * allocates a socket_info_t array, and fills each entry with:
 *   - Protocol (TCP/UDP)
 *   - Address family (AF_INET/AF_INET6)
 *   - Kernel virtual address (kvaddr)
 *   - IP address and port
 *
 * Returns: 0 on success, -1 on failure (errno is set accordingly):
 *            - ENOMEM if memory allocation fails
 *            - other errno set by sysctl_fetch
 */
static int fetch_sockets_common(struct socket_info_t **list, size_t *nentries, int mib[], unsigned int mib_size)
{
    char *buffer;
    size_t buffer_size;

    if (sysctl_fetch(mib, mib_size, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    struct kinfo_pcb *sockets = (struct kinfo_pcb *)buffer;
    size_t socknum = buffer_size / sizeof(struct kinfo_pcb);

    *nentries = count_sockets(sockets, socknum);
    if (*nentries == 0)
    {
        free(buffer);
        return 0;
    }

    *list = malloc(*nentries * sizeof(struct socket_info_t));
    if (!*list)
    {
        free(buffer);
        errno = ENOMEM;
        return -1;
    }

    size_t idx = 0;
    for (size_t i = 0; i < socknum; ++i)
    {
        if (sockets[i].ki_protocol != IPPROTO_TCP && sockets[i].ki_protocol != IPPROTO_UDP)
            continue;

        (*list)[idx].protocol = sockets[i].ki_protocol;
        (*list)[idx].address.family = sockets[i].ki_family;
        (*list)[idx].kvaddr = sockets[i].ki_sockaddr;

        if (sockets[i].ki_family == AF_INET)
        {
            struct sockaddr_in *sin = (struct sockaddr_in *)(&sockets[i].ki_src);
            (*list)[idx].address.addr.ipv4 = sin->sin_addr;
            (*list)[idx].port = ntohs(sin->sin_port);
        }
        else
        {
            struct sockaddr_in6 *sin6 = (struct sockaddr_in6 *)(&sockets[i].ki_src);
            (*list)[idx].address.addr.ipv6 = sin6->sin6_addr;
            (*list)[idx].port = ntohs(sin6->sin6_port);
        }

        ++idx;
    }

    free(buffer);
    return 0;
}

int netbsd_fetch_tcp_sockets(struct socket_info_t **list, size_t *nentries)
{
    unsigned int mib_len;
    int mib[CTL_MAXNAME];

    if (sysctlgetmibinfo("net.inet.tcp.pcblist", &mib[0], &mib_len, NULL, NULL, NULL, SYSCTL_VERSION) != 0)
    {
        return -1;
    }

    mib[mib_len++] = PCB_ALL;
    mib[mib_len++] = 0;
    mib[mib_len++] = sizeof(struct kinfo_pcb);
    mib[mib_len++] = INT_MAX;

    return fetch_sockets_common(list, nentries, mib, mib_len);
}

int netbsd_fetch_tcp6_sockets(struct socket_info_t **list, size_t *nentries)
{
    unsigned int mib_len;
    int mib[CTL_MAXNAME];

    if (sysctlgetmibinfo("net.inet6.tcp6.pcblist", &mib[0], &mib_len, NULL, NULL, NULL, SYSCTL_VERSION) != 0)
    {
        return -1;
    }

    mib[mib_len++] = PCB_ALL;
    mib[mib_len++] = 0;
    mib[mib_len++] = sizeof(struct kinfo_pcb);
    mib[mib_len++] = INT_MAX;

    return fetch_sockets_common(list, nentries, mib, mib_len);
}

int netbsd_fetch_udp_sockets(struct socket_info_t **list, size_t *nentries)
{
    unsigned int mib_len;
    int mib[CTL_MAXNAME];

    if (sysctlgetmibinfo("net.inet.udp.pcblist", &mib[0], &mib_len, NULL, NULL, NULL, SYSCTL_VERSION) != 0)
    {
        return -1;
    }

    mib[mib_len++] = PCB_ALL;
    mib[mib_len++] = 0;
    mib[mib_len++] = sizeof(struct kinfo_pcb);
    mib[mib_len++] = INT_MAX;

    return fetch_sockets_common(list, nentries, mib, mib_len);
}

int netbsd_fetch_udp6_sockets(struct socket_info_t **list, size_t *nentries)
{
    unsigned int mib_len;
    int mib[CTL_MAXNAME];

    if (sysctlgetmibinfo("net.inet6.udp6.pcblist", &mib[0], &mib_len, NULL, NULL, NULL, SYSCTL_VERSION) != 0)
    {
        return -1;
    }

    mib[mib_len++] = PCB_ALL;
    mib[mib_len++] = 0;
    mib[mib_len++] = sizeof(struct kinfo_pcb);
    mib[mib_len++] = INT_MAX;

    return fetch_sockets_common(list, nentries, mib, mib_len);
}

int netbsd_fetch_socket_files(struct socket_file_t **list, size_t *nentries)
{
    unsigned int mib_len;
    int mib[CTL_MAXNAME];

    if (sysctlgetmibinfo("kern.file2", &mib[0], &mib_len, NULL, NULL, NULL, SYSCTL_VERSION) != 0)
    {
        return -1;
    }

    mib[mib_len++] = KERN_FILE_BYPID;
    mib[mib_len++] = 0;
    mib[mib_len++] = sizeof(struct kinfo_file);
    mib[mib_len++] = INT_MAX;

    char *buffer;
    size_t buffer_size;

    if (sysctl_fetch(mib, mib_len, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    struct kinfo_file *files = (struct kinfo_file *)buffer;
    size_t filesnum = buffer_size / sizeof(struct kinfo_file);

    *nentries = count_files(files, filesnum);

    if (*nentries == 0)
    {
        free(buffer);
        return 0;
    }

    *list = malloc(*nentries * sizeof(struct socket_file_t));
    if (!*list)
    {
        free(buffer);
        errno = ENOMEM;
        return -1;
    }

    size_t idx = 0;
    for (size_t i = 0; i < filesnum; ++i)
    {
        if (files[i].ki_ftype != DTYPE_SOCKET)
            continue;

        (*list)[idx].pid = files[i].ki_pid;
        (*list)[idx].kvaddr = files[i].ki_fdata;

        idx++;
    }

    free(buffer);
    return 0;
}

char *netbsd_fetch_process_name(pid_t pid)
{
    int mib[6] = {CTL_KERN, KERN_PROC2, KERN_PROC_PID, pid, sizeof(struct kinfo_proc2), 1};
    struct kinfo_proc2 proc;
    size_t size = sizeof(proc);

    if (sysctl(mib, 6, &proc, &size, NULL, 0) < 0)
        return NULL;

    char *name = strdup(proc.p_comm);
    if (!name)
        errno = ENOMEM;

    return name;
}

char *netbsd_fetch_process_path(pid_t pid)
{
    int mib[4] = {CTL_KERN, KERN_PROC_ARGS, pid, KERN_PROC_PATHNAME};
    char pathname[PATH_MAX];
    size_t size = sizeof(pathname);

    if (sysctl(mib, 4, pathname, &size, NULL, 0) < 0)
        return NULL;

    char *path = strdup(pathname);
    if (!path)
        errno = ENOMEM;

    return path;
}

#endif