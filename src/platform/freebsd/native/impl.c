#include "impl.h"

static size_t count_entries(struct xinpgen *xig)
{
    size_t retval = 0;

    for (xig = (struct xinpgen *)((char *)xig + xig->xig_len);
         xig->xig_len != sizeof(struct xinpgen);
         xig = (struct xinpgen *)((char *)xig + xig->xig_len))
        ++retval;

    return retval;
}

static int get_pcb_list(int mib[], int mnum, char **buffer, size_t *buffer_size)
{
    if (sysctl(mib, mnum, NULL, buffer_size, NULL, 0) < 0)
        return -1;

    *buffer = malloc(*buffer_size);
    if (!*buffer)
    {
        errno = ENOMEM;
        return -1;
    }

    if (sysctl(mib, mnum, *buffer, buffer_size, NULL, 0) < 0)
    {
        free(*buffer);
        return -1;
    }

    return 0;
}

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

static int lsock_impl(struct socket_info_t **list, size_t *nentries, int protocol)
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

    if (get_pcb_list(mib, 4, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    struct xinpgen *xig = (struct xinpgen *)buffer;
    *nentries = count_entries(xig);

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

int lsock_tcp(struct socket_info_t **list, size_t *nentries)
{
    return lsock_impl(list, nentries, IPPROTO_TCP);
}

int lsock_udp(struct socket_info_t **list, size_t *nentries)
{
    return lsock_impl(list, nentries, IPPROTO_UDP);
}

int lsock_files(struct socket_file_t **list, size_t *nentries)
{
    int mib[2];
    mib[0] = CTL_KERN;
    mib[1] = KERN_FILE;

    char *buffer;
    size_t buffer_size;

    if (get_pcb_list(mib, 2, &buffer, &buffer_size) < 0)
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