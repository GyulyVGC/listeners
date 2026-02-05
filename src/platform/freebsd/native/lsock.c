#include "lsock.h"

static size_t count_entries(struct xinpgen *xig, int vflag, enum protocol_t protocol)
{
    size_t retval = 0;

    for (xig = (struct xinpgen *)((char *)xig + xig->xig_len);
         xig->xig_len != sizeof(struct xinpgen);
         xig = (struct xinpgen *)((char *)xig + xig->xig_len))
    {
        if (protocol == PROTOCOL_TCP)
        {
            struct xtcpcb *xtp = (struct xtcpcb *)xig;

            if (xtp->t_state == TCPS_LISTEN && xtp->xt_inp.inp_vflag & vflag)
                ++retval;
        }
        else
        {
            struct xinpcb *xip = (struct xinpcb *)xig;

            if (xip->inp_vflag & vflag)
                ++retval;
        }
    }

    return retval;
}

static int get_pcb_list(int mib[4], char **buffer, size_t *buffer_size)
{
    if (sysctl(mib, 4, NULL, buffer_size, NULL, 0) < 0)
        return -1;

    *buffer = malloc(*buffer_size);
    if (!*buffer)
    {
        errno = ENOMEM;
        return -1;
    }

    if (sysctl(mib, 4, *buffer, buffer_size, NULL, 0) < 0)
    {
        free(*buffer);
        return -1;
    }

    return 0;
}

static int lsock_impl(struct socket_info_t **list, size_t *nentries, enum protocol_t protocol, int is_ipv6)
{
    if (list == NULL || nentries == NULL)
    {
        errno = EINVAL;
        return -1;
    }

    int vflag = is_ipv6 ? INP_IPV6 : INP_IPV4;

    int mib[4];
    mib[0] = CTL_NET;
    mib[1] = PF_INET;
    mib[2] = protocol == PROTOCOL_TCP ? IPPROTO_TCP : IPPROTO_UDP;
    mib[3] = protocol == PROTOCOL_TCP ? TCPCTL_PCBLIST : UDPCTL_PCBLIST;

    char *buffer;
    size_t buffer_size;

    if (get_pcb_list(mib, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    struct xinpgen *xig = (struct xinpgen *)buffer;
    *nentries = count_entries(xig, vflag, protocol);

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

        if (protocol == PROTOCOL_TCP)
        {
            struct xtcpcb *xtp = (struct xtcpcb *)xig;

            if (xtp->t_state == TCPS_LISTEN && xtp->xt_inp.inp_vflag & vflag)
            {

                (*list)[index].port = ntohs(xtp->xt_inp.inp_lport);
                (*list)[index].protocol = PROTOCOL_TCP;

                if (is_ipv6)
                {
                    (*list)[index].address.addr.ipv6 = xtp->xt_inp.in6p_laddr;
                    (*list)[index].address.family = AF_INET6;
                }
                else
                {
                    (*list)[index].address.addr.ipv4 = xtp->xt_inp.inp_laddr;
                    (*list)[index].address.family = AF_INET;
                }

                ++index;
            }
        }
        else
        {
            struct xinpcb *xip = (struct xinpcb *)xig;

            if (xip->inp_vflag & vflag)
            {
                (*list)[index].port = ntohs(xip->inp_lport);
                (*list)[index].protocol = PROTOCOL_UDP;

                if (is_ipv6)
                {
                    (*list)[index].address.addr.ipv6 = xip->in6p_laddr;
                    (*list)[index].address.family = AF_INET6;
                }
                else
                {
                    (*list)[index].address.addr.ipv4 = xip->inp_laddr;
                    (*list)[index].address.family = AF_INET;
                }

                ++index;
            }
        }
    }

    free(buffer);
    return 0;
}

int lsock_tcp(struct socket_info_t **list, size_t *nentries)
{
    return lsock_impl(list, nentries, PROTOCOL_TCP, 0);
}

int lsock_tcp6(struct socket_info_t **list, size_t *nentries)
{
    return lsock_impl(list, nentries, PROTOCOL_TCP, 1);
}

int lsock_udp(struct socket_info_t **list, size_t *nentries)
{
    return lsock_impl(list, nentries, PROTOCOL_UDP, 0);
}

int lsock_udp6(struct socket_info_t **list, size_t *nentries)
{
    return lsock_impl(list, nentries, PROTOCOL_UDP, 1);
}