#pragma once

#ifndef LYCANTHROPE_RAWSOCK_H
#define LYCANTHROPE_RAWSOCK_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif


int lyc_network_init(void);
void lyc_network_cleanup(void);
int64_t lyc_raw_socket_create(void);
int lyc_raw_socket_send(int64_t sock,const uint8_t *packet,size_t len,uint32_t dst_ip,uint16_t dst_port);
void lyc_raw_socket_close(int64_t sock);
const char *lyc_last_error(void);

#ifdef __cplusplus
}
#endif

#endif /* LYCANTHROPE_RAWSOCK_H */


