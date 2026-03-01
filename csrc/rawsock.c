#include <string.h>
#include <stdio.h>

#include "rawsock.h"



#ifdef _WIN32
    static __declspec(thread) char lyc_errbuf[256];
#else
    static __thread char lyc_errbuf[256];
#endif

static void lyc_set_error(const char *msg)
{
    strncpy(lyc_errbuf, msg, sizeof(lyc_errbuf) - 1);
    lyc_errbuf[sizeof(lyc_errbuf) - 1] = '\0';
}

const char *lyc_last_error(void)
{
    return lyc_errbuf;
}


#ifdef _WIN32

#include <winsock2.h>
#include <ws2tcpip.h>

#pragma comment(lib, "ws2_32.lib")

int lyc_network_init(void)
{
    WSADATA wsa;
    int ret = WSAStartup(MAKEWORD(2, 2), &wsa);

    if (ret != 0) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"WSAStartup failed: %d", ret);
        return -1;
    }
    return 0;
}

void lyc_network_cleanup(void)
{
    WSACleanup();
}

int64_t lyc_raw_socket_create(void)
{
    SOCKET sock;
    DWORD optval = 1;
    sock = WSASocket(AF_INET, SOCK_RAW, IPPROTO_RAW, NULL, 0, WSA_FLAG_OVERLAPPED);

    if (sock == INVALID_SOCKET) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"WSASocket failed: %d", WSAGetLastError());
        return -1;
    }

    if (setsockopt(sock, IPPROTO_IP, IP_HDRINCL,(const char *)&optval, sizeof(optval)) == SOCKET_ERROR) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"setsockopt IP_HDRINCL failed: %d", WSAGetLastError());
        closesocket(sock);
        return -1;
    }
    return (int64_t)sock;
}

int lyc_raw_socket_send(int64_t sock, const uint8_t *packet,size_t len, uint32_t dst_ip, uint16_t dst_port)
{
    struct sockaddr_in addr;
    int ret;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family      = AF_INET;
    addr.sin_port        = htons(dst_port);
    addr.sin_addr.s_addr = dst_ip;
    ret = sendto((SOCKET)sock, (const char *)packet, (int)len, 0,(struct sockaddr *)&addr, sizeof(addr));

    if (ret == SOCKET_ERROR) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"sendto failed: %d", WSAGetLastError());
        return -1;
    }
    return ret;
}

void lyc_raw_socket_close(int64_t sock)
{
    if (sock >= 0) 
    {
        closesocket((SOCKET)sock);
    }
}


#else

#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <errno.h>

int lyc_network_init(void)
{
    return 0;
}

void lyc_network_cleanup(void)
{

}

int64_t lyc_raw_socket_create(void)
{
    int sock;
    int optval = 1;
    sock = socket(AF_INET, SOCK_RAW, IPPROTO_RAW);

    if (sock < 0) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"socket() failed: %s (need root/sudo)", strerror(errno));
        return -1;
    }

    if (setsockopt(sock, IPPROTO_IP, IP_HDRINCL,&optval, sizeof(optval)) < 0) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"setsockopt IP_HDRINCL: %s", strerror(errno));
        close(sock);
        return -1;
    }


#if defined(__APPLE__)
    {
        int nosigpipe = 1;
        setsockopt(sock, SOL_SOCKET, SO_NOSIGPIPE, &nosigpipe, sizeof(nosigpipe));
    }
#endif
    return (int64_t)sock;
}

int lyc_raw_socket_send(int64_t sock, const uint8_t *packet,size_t len, uint32_t dst_ip, uint16_t dst_port)
{
    struct sockaddr_in addr;
    ssize_t ret;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family      = AF_INET;
    addr.sin_port        = htons(dst_port);
    addr.sin_addr.s_addr = dst_ip;
    int flags = 0;
#if defined(__linux__) || defined(__ANDROID__)
    flags = MSG_NOSIGNAL;
#endif

    ret = sendto((int)sock, packet, len, flags,(struct sockaddr *)&addr, sizeof(addr));

    if (ret < 0) 
    {
        snprintf(lyc_errbuf, sizeof(lyc_errbuf),"sendto failed: %s", strerror(errno));
        return -1;
    }
    return (int)ret;
}

void lyc_raw_socket_close(int64_t sock)
{
    if (sock >= 0) 
    {
        close((int)sock);
    }
}

#endif /* _WIN32 */