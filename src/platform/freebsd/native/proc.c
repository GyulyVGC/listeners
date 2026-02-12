#include "proc.h"

int proc_list(struct process_info_t **list, size_t *nentries)
{
    int mib[3] = {CTL_KERN, KERN_PROC, KERN_PROC_PROC};
    size_t buflen = 0;

    if (sysctl(mib, 3, NULL, &buflen, NULL, 0) == -1)
    {
        return -1;
    }

    if (buflen == 0)
    {
        *list = NULL;
        *nentries = 0;
        return 0;
    }

    struct kinfo_proc *procbuf = malloc(buflen);
    if (!procbuf)
    {
        errno = ENOMEM;
        return -1;
    }

    if (sysctl(mib, 3, procbuf, &buflen, NULL, 0) == -1)
    {
        free(procbuf);
        return -1;
    }

    *nentries = buflen / sizeof(struct kinfo_proc);

    *list = calloc(*nentries, sizeof(struct process_info_t));
    if (!*list)
    {
        errno = ENOMEM;
        free(procbuf);
        return -1;
    }

    for (size_t i = 0; i < *nentries; i++)
    {
        (*list)[i].pid = procbuf[i].ki_pid;
        strlcpy((*list)[i].name, procbuf[i].ki_comm, sizeof((*list)[i].name));

        int mib_path[4] = {CTL_KERN, KERN_PROC, KERN_PROC_PATHNAME, procbuf[i].ki_pid};
        size_t pathlen = sizeof((*list)[i].path);
        if (sysctl(mib_path, 4, (*list)[i].path, &pathlen, NULL, 0) == -1)
        {
            (*list)[i].path[0] = '\0';
        }
    }

    free(procbuf);
    return 0;
}

int proc_sockets(pid_t pid, struct socket_info_t **list, size_t *nentries)
{
    int cnt;

    struct kinfo_file *kf = kinfo_getfile(pid, &cnt);
    if (kf == NULL)
    {
        *list = NULL;
        *nentries = 0;
        return -1;
    }

    int retval_cnt = 0;
    for (int i = 0; i < cnt; i++)
    {
        if (kf[i].kf_type != KF_TYPE_SOCKET)
            continue;

        if (kf[i].kf_sock_protocol == IPPROTO_UDP || kf[i].kf_sock_protocol == IPPROTO_TCP)
            retval_cnt++;
    }

    if (retval_cnt == 0)
    {
        free(kf);
        *list = NULL;
        *nentries = 0;
        return 0;
    }

    *list = calloc(retval_cnt, sizeof(struct socket_info_t));
    if (!*list)
    {
        free(kf);
        *list = NULL;
        *nentries = 0;
        errno = ENOMEM;
        return -1;
    }

    int idx = 0;
    for (int i = 0; i < cnt; i++)
    {
        if (kf[i].kf_type != KF_TYPE_SOCKET)
            continue;

        if (kf[i].kf_sock_protocol != IPPROTO_UDP && kf[i].kf_sock_protocol != IPPROTO_TCP)
            continue;

        (*list)[idx].protocol = (kf[i].kf_sock_protocol == IPPROTO_UDP ? PROTOCOL_UDP : PROTOCOL_TCP);

        struct sockaddr_storage *local = &kf[i].kf_sa_local;
        if (local->ss_family == AF_INET)
        {
            struct sockaddr_in *s = (struct sockaddr_in *)local;

            (*list)[idx].address.family = AF_INET;
            (*list)[idx].address.addr.ipv4 = s->sin_addr;
            (*list)[idx].port = ntohs(s->sin_port);

            ++idx;
        }
        else if (local->ss_family == AF_INET6)
        {
            struct sockaddr_in6 *s6 = (struct sockaddr_in6 *)local;

            (*list)[idx].address.family = AF_INET6;
            (*list)[idx].address.addr.ipv6 = s6->sin6_addr;
            (*list)[idx].port = ntohs(s6->sin6_port);

            ++idx;
        }
    }

    free(kf);

    *nentries = retval_cnt;
    return 0;
}