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

static int count_sockets(struct kinfo_file *files, size_t size)
{
    size_t retval = 0;

    for (size_t i = 0; i < size; ++i)
    {
        if (files[i].f_type != DTYPE_SOCKET)
            continue;

        if (files[i].so_protocol != IPPROTO_TCP && files[i].so_protocol != IPPROTO_UDP)
            continue;

        ++retval;
    }

    return retval;
}

int proc_all(struct proc_info_t **list, size_t *nentries)
{
    int mib[6] = {CTL_KERN, KERN_PROC, KERN_PROC_ALL, 0, sizeof(struct kinfo_proc), INT_MAX};

    char *buffer;
    size_t buffer_size;

    if (sysctl_call(mib, 6, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    *nentries = buffer_size / sizeof(struct kinfo_proc);
    *list = calloc(*nentries, sizeof(struct proc_info_t));

    if (!*list)
    {
        free(buffer);
        errno = ENOMEM;
        return -1;
    }

    struct kinfo_proc *procs = (struct kinfo_proc *)buffer;
    for (size_t i = 0; i < *nentries; ++i)
    {
        (*list)[i].pid = procs[i].p_pid;
        strncpy((*list)[i].name, procs[i].p_comm, KI_MAXCOMLEN);
    }

    free(buffer);
    return 0;
}

int socks_by_pid(pid_t pid, struct socket_info_t **list, size_t *nentries)
{
    int mib[6] = {CTL_KERN, KERN_FILE, KERN_FILE_BYPID, pid, sizeof(struct kinfo_file), INT_MAX};

    char *buffer;
    size_t buffer_size;

    if (sysctl_call(mib, 6, &buffer, &buffer_size) < 0)
    {
        return -1;
    }

    size_t files_num = buffer_size / sizeof(struct kinfo_file);
    struct kinfo_file *files = (struct kinfo_file *)buffer;

    *nentries = count_sockets(files, files_num);

    if (*nentries == 0)
    {
        free(buffer);
        return 0;
    }

    *list = malloc(*nentries * sizeof(struct proc_info_t));

    size_t idx = 0;
    for (size_t i = 0; i < files_num; ++i)
    {
        if (files[i].f_type != DTYPE_SOCKET)
            continue;

        if (files[i].so_protocol != IPPROTO_TCP && files[i].so_protocol != IPPROTO_UDP)
            continue;

        (*list)[idx].protocol = files[i].so_protocol;
        (*list)[idx].port = ntohs(files[i].inp_lport);
        (*list)[idx].address.family = files[i].so_family;
        memcpy(&((*list)[idx].address.addr), files[i].inp_laddru, sizeof(files[i].inp_laddru));

        ++idx;
    }

    free(buffer);
    return 0;
}