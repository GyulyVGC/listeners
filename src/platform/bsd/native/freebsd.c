#include "freebsd.h"

#ifdef __FreeBSD__

/**
 * count_sockets - Count the number of sockets in a sysctl xinpgen buffer
 *
 * @xig: Pointer to the start of a sysctl buffer returned for TCP/UDP PCBs.
 *
 * This function iterates over the xinpgen-linked structures returned by
 * FreeBSD sysctl calls for TCP or UDP protocol control blocks, counting
 * the number of socket entries.
 *
 * Returns: The number of socket entries found.
 */
static size_t count_sockets(struct xinpgen *xig)
{
    size_t retval = 0;

    for (xig = (struct xinpgen *)((char *)xig + xig->xig_len);
         xig->xig_len != sizeof(struct xinpgen);
         xig = (struct xinpgen *)((char *)xig + xig->xig_len))
        ++retval;

    return retval;
}

/**
 * fillsock_tcp - Fill a socket_info_t structure from a TCP xtcpcb entry
 *
 * @sock: Pointer to the socket_info_t structure to fill.
 * @xtp:  Pointer to the TCP control block (xtcpcb) returned by sysctl.
 *
 * Sets the port, protocol, kernel virtual address, and IP address
 * (v4 or v6) of the socket.
 */
static void fillsock_tcp(struct socket_info_t *sock, struct xtcpcb *xtp)
{
    sock->port = ntohs(xtp->xt_inp.inp_lport);
    sock->protocol = IPPROTO_TCP;
    sock->kvaddr = xtp->xt_inp.xi_socket.xso_so;

    if (xtp->xt_inp.inp_vflag & INP_IPV6)
    {
        sock->address.addr.ipv6 = xtp->xt_inp.in6p_laddr;
        sock->address.family = AF_INET6;
    }
    else
    {
        sock->address.addr.ipv4 = xtp->xt_inp.inp_laddr;
        sock->address.family = AF_INET;
    }
}

/**
 * fillsock_udp - Fill a socket_info_t structure from a UDP xinpcb entry
 *
 * @sock: Pointer to the socket_info_t structure to fill.
 * @xip:  Pointer to the UDP control block (xinpcb) returned by sysctl.
 *
 * Sets the port, protocol, kernel virtual address, and IP address
 * (v4 or v6) of the socket.
 */
static void fillsock_udp(struct socket_info_t *sock, struct xinpcb *xip)
{
    sock->port = ntohs(xip->inp_lport);
    sock->protocol = IPPROTO_UDP;
    sock->kvaddr = xip->xi_socket.xso_so;

    if (xip->inp_vflag & INP_IPV6)
    {
        sock->address.addr.ipv6 = xip->in6p_laddr;
        sock->address.family = AF_INET6;
    }
    else
    {
        sock->address.addr.ipv4 = xip->inp_laddr;
        sock->address.family = AF_INET;
    }
}

/**
 * fetch_sockets_common - Fetch all TCP or UDP sockets and fill socket_info_t list
 *
 * @list:     Output pointer that will point to a dynamically allocated array
 *            of socket_info_t. Caller is responsible for freeing it.
 * @nentries: Output number of entries in the list.
 * @protocol: Either IPPROTO_TCP or IPPROTO_UDP.
 *
 * This function queries the kernel using sysctl to get the list of TCP
 * or UDP sockets, counts entries, allocates a socket_info_t array, and
 * fills it using fillsock_tcp/fillsock_udp.
 *
 * Returns: 0 on success, -1 on failure (errno is set accordingly):
 *            - EINVAL if list or nentries is NULL
 *            - ENOMEM if memory allocation fails
 *            - other errno set by sysctl_fetch()
 */
static int fetch_sockets_common(struct socket_info_t **list, size_t *nentries, int protocol)
{
    if (list == NULL || nentries == NULL)
    {
        errno = EINVAL;
        return -1;
    }

    int mib[4];
    mib[0] = CTL_NET;
    mib[1] = PF_INET;
    mib[2] = protocol;
    mib[3] = protocol == IPPROTO_TCP ? TCPCTL_PCBLIST : UDPCTL_PCBLIST;

    char *buffer;
    size_t buffer_size;

    if (sysctl_fetch(mib, 4, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    struct xinpgen *xig = (struct xinpgen *)buffer;
    *nentries = count_sockets(xig);

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

    size_t index = 0;
    for (xig = (struct xinpgen *)((char *)xig + xig->xig_len);
         xig->xig_len != sizeof(struct xinpgen);
         xig = (struct xinpgen *)((char *)xig + xig->xig_len))
    {

        if (protocol == IPPROTO_TCP)
        {
            struct xtcpcb *xtp = (struct xtcpcb *)xig;
            fillsock_tcp(&((*list)[index]), xtp);
            ++index;
        }
        else
        {
            struct xinpcb *xip = (struct xinpcb *)xig;
            fillsock_udp(&((*list)[index]), xip);
            ++index;
        }
    }

    free(buffer);
    return 0;
}

int freebsd_fetch_tcp_sockets(struct socket_info_t **list, size_t *nentries)
{
    return fetch_sockets_common(list, nentries, IPPROTO_TCP);
}

int freebsd_fetch_udp_sockets(struct socket_info_t **list, size_t *nentries)
{
    return fetch_sockets_common(list, nentries, IPPROTO_UDP);
}

int freebsd_fetch_socket_files(struct socket_file_t **list, size_t *nentries)
{
    int mib[2];
    mib[0] = CTL_KERN;
    mib[1] = KERN_FILE;

    char *buffer;
    size_t buffer_size;

    if (sysctl_fetch(mib, 2, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    struct xfile *xfiles = (struct xfile *)buffer;
    size_t filesnum = buffer_size / sizeof(struct xfile);

    size_t socknum = 0;
    for (size_t i = 0; i < filesnum; ++i)
    {
        if (xfiles[i].xf_type == DTYPE_SOCKET)
            ++socknum;
    }

    if (socknum == 0)
    {
        free(buffer);
        *nentries = 0;
        return 0;
    }

    *list = malloc(socknum * sizeof(struct socket_file_t));
    if (!*list)
    {
        free(buffer);
        errno = ENOMEM;
        return -1;
    }

    size_t idx = 0;
    for (size_t i = 0; i < filesnum; ++i)
    {
        if (xfiles[i].xf_type == DTYPE_SOCKET)
        {
            (*list)[idx].pid = xfiles[i].xf_pid;
            (*list)[idx].kvaddr = xfiles[i].xf_data;

            idx++;
        }
    }

    *nentries = socknum;
    free(buffer);
    return 0;
}

char *freebsd_fetch_process_name(pid_t pid)
{
    int mib[4] = {CTL_KERN, KERN_PROC, KERN_PROC_PID, pid};
    struct kinfo_proc proc;
    size_t size = sizeof(proc);

    if (sysctl(mib, 4, &proc, &size, NULL, 0) < 0)
        return NULL;

    char *name = strdup(proc.ki_comm);
    if (!name)
        errno = ENOMEM;

    return name;
}

char *freebsd_fetch_process_path(pid_t pid)
{
    int mib[4] = {CTL_KERN, KERN_PROC, KERN_PROC_PATHNAME, pid};
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