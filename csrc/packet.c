#include <string.h>

#include <stdlib.h>

#include "packet.h"
#include "checksum.h"

#ifdef _WIN32
    #include <winsock2.h>
#else
    #include <arpa/inet.h>
#endif

LYC_PACKED_BEGIN
typedef struct {
    uint8_t  ver_ihl;
    uint8_t  tos;
    uint16_t total_len;
    uint16_t id;
    uint16_t flags_frag;
    uint8_t  ttl;
    uint8_t  protocol;
    uint16_t checksum;
    uint32_t src_ip;
    uint32_t dst_ip;
} LYC_PACKED_ATTR IpHeader;
LYC_PACKED_END

LYC_PACKED_BEGIN
typedef struct {
    uint16_t src_port;
    uint16_t dst_port;
    uint32_t seq;
    uint32_t ack;
    uint8_t  data_offset;
    uint8_t  flags;
    uint16_t window;
    uint16_t checksum;
    uint16_t urgent;
} LYC_PACKED_ATTR TcpHeader;
LYC_PACKED_END

int lyc_build_tcp_packet(const TcpPacketParams *params,const uint8_t *payload,size_t payload_len,uint8_t *out_buffer,size_t buffer_size)
{
    size_t ip_hdr_len  = sizeof(IpHeader);    // 20
    size_t tcp_hdr_len = sizeof(TcpHeader);   // 20
    size_t total_len   = ip_hdr_len + tcp_hdr_len + payload_len;

    if (total_len > buffer_size || total_len > 65535) 
    {
        return -1;
    }
    memset(out_buffer, 0, total_len);
    IpHeader *ip = (IpHeader *)out_buffer;
    ip->ver_ihl    = 0x45; 
    ip->tos        = 0;
    ip->total_len  = htons((uint16_t)total_len);
    ip->id         = htons((uint16_t)(rand() & 0xFFFF));
    ip->flags_frag = htons(0x4000);
    ip->ttl        = params->ttl > 0 ? params->ttl : 64;
    ip->protocol   = 6;  // TCP
    ip->checksum   = 0;
    ip->src_ip     = params->src_ip;
    ip->dst_ip     = params->dst_ip;
    ip->checksum   = lyc_ip_checksum(out_buffer, ip_hdr_len);
    TcpHeader *tcp = (TcpHeader *)(out_buffer + ip_hdr_len);
    tcp->src_port    = htons(params->src_port);
    tcp->dst_port    = htons(params->dst_port);
    tcp->seq         = htonl(params->seq);
    tcp->ack         = htonl(params->ack);
    tcp->data_offset = 0x50;  // 5 * 4 = 20 bytes
    tcp->flags       = params->flags;
    tcp->window      = htons(params->window > 0 ? params->window : 65535);
    tcp->checksum    = 0;
    tcp->urgent      = 0;

    if (payload_len > 0 && payload != NULL) 
    {
        memcpy(out_buffer + ip_hdr_len + tcp_hdr_len,payload, payload_len);
    }
    tcp->checksum = lyc_tcp_checksum(params->src_ip,params->dst_ip,out_buffer + ip_hdr_len,tcp_hdr_len + payload_len);
    return (int)total_len;
}
