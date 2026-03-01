#pragma once

#ifndef LYCANTHROPE_CHECKSUM_H
#define LYCANTHROPE_CHECKSUM_H

#include <stdint.h>
#include <stddef.h>

#ifdef _WIN32
    #include <winsock2.h>
#else
    #include <arpa/inet.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

uint16_t lyc_ip_checksum(const uint8_t *header, size_t len);
uint16_t lyc_tcp_checksum(uint32_t src_ip, uint32_t dst_ip,const uint8_t *tcp_segment, size_t tcp_len);

#ifdef __cplusplus
}
#endif

#endif /* LYCANTHROPE_CHECKSUM_H */




