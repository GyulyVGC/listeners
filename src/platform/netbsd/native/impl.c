#include "impl.h"

static int sysctl_call(int mib[], int mnum, char **buffer, size_t *buffer_size)
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

static int lsock_common(struct socket_info_t **list, size_t *nentries, int mib[], unsigned int mib_size)
{
    char *buffer;
    size_t buffer_size;

    if (sysctl_call(mib, mib_size, &buffer, &buffer_size) < 0)
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

int lsock_tcp(struct socket_info_t **list, size_t *nentries)
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

    return lsock_common(list, nentries, mib, mib_len);
}

int lsock_tcp6(struct socket_info_t **list, size_t *nentries)
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

    return lsock_common(list, nentries, mib, mib_len);
}

int lsock_udp(struct socket_info_t **list, size_t *nentries)
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

    return lsock_common(list, nentries, mib, mib_len);
}

int lsock_udp6(struct socket_info_t **list, size_t *nentries)
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

    return lsock_common(list, nentries, mib, mib_len);
}

int lsock_files(struct socket_file_t **list, size_t *nentries)
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

    if (sysctl_call(mib, mib_len, &buffer, &buffer_size) < 0)
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

char *proc_name(pid_t pid)
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

char *proc_path(pid_t pid)
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