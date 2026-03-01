#pragma once


#ifndef LYCANTHROPE_PLATFORM_H
#define LYCANTHROPE_PLATFORM_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>


#ifdef _WIN32
    #ifndef WIN32_LEAN_AND_MEAN
        #define WIN32_LEAN_AND_MEAN
    #endif
    #include <winsock2.h>
    #include <ws2tcpip.h>

    #ifdef _MSC_VER
        #pragma comment(lib, "ws2_32.lib")
    #endif

    typedef SOCKET lyc_os_socket_t;
    #define LYC_OS_INVALID_SOCKET  INVALID_SOCKET
    #define LYC_OS_CLOSE_SOCKET    closesocket
    #define LYC_OS_SOCKET_ERROR    SOCKET_ERROR

#else
    #include <sys/types.h>
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <netinet/ip.h>
    #include <netinet/tcp.h>
    #include <arpa/inet.h>
    #include <unistd.h>
    #include <errno.h>

    typedef int lyc_os_socket_t;
    #define LYC_OS_INVALID_SOCKET  (-1)
    #define LYC_OS_CLOSE_SOCKET    close
    #define LYC_OS_SOCKET_ERROR    (-1)
#endif


#ifdef _MSC_VER
    #define LYC_PACKED_BEGIN  __pragma(pack(push, 1))
    #define LYC_PACKED_END    __pragma(pack(pop))
    #define LYC_PACKED_ATTR
#else
    #define LYC_PACKED_BEGIN
    #define LYC_PACKED_END
    #define LYC_PACKED_ATTR  __attribute__((packed))
#endif


#ifndef IP_HDRINCL
    #ifdef _WIN32
        #define IP_HDRINCL  2
    #else
        #define IP_HDRINCL  3
    #endif
#endif


static inline int lyc_last_error(void)
{
#ifdef _WIN32
    return WSAGetLastError();
#else
    return errno;
#endif
}

#endif /* LYCANTHROPE_PLATFORM_H */