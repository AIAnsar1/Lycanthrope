#include "checksum.h"
#include <string.h>

#ifdef _MSC_VER
    #pragma pack(push, 1)
#endif

typedef struct {
    uint32_t src;
    uint32_t dst;
    uint8_t  zero;
    uint8_t  protocol;
    uint16_t tcp_len;
}
#ifdef _MSC_VER
PseudoHeader;
    #pragma pack(pop)
#else
__attribute__((packed)) PseudoHeader;
#endif

uint16_t lyc_ip_checksum(const uint8_t *header, size_t header_len)
{
    uint32_t sum = 0;
    const uint16_t *ptr = (const uint16_t *)header;
    size_t count = header_len;

    while (count > 1) 
    {
        sum += *ptr++;
        count -= 2;
    }

    if (count == 1) 
    {
        uint16_t last = 0;
        memcpy(&last, ptr, 1);
        sum += last;
    }
    sum = (sum >> 16) + (sum & 0xFFFF);
    sum += (sum >> 16);
    return (uint16_t)(~sum);
}

uint16_t lyc_tcp_checksum(uint32_t src_ip, uint32_t dst_ip, const uint8_t *tcp_segment, size_t tcp_len)
{
    PseudoHeader pseudo;
    uint32_t sum = 0;
    const uint16_t *p;
    size_t count;
    size_t i;
    pseudo.src      = src_ip;
    pseudo.dst      = dst_ip;
    pseudo.zero     = 0;
    pseudo.protocol = 6;
    pseudo.tcp_len  = htons((uint16_t)tcp_len);
    p = (const uint16_t *)&pseudo;

    for (i = 0; i < sizeof(pseudo) / 2; i++) 
    {
        sum += p[i];
    }
    p = (const uint16_t *)tcp_segment;
    count = tcp_len;

    while (count > 1) 
    {
        sum += *p++;
        count -= 2;
    }

    if (count == 1) 
    {
        uint16_t last = 0;
        memcpy(&last, p, 1);
        sum += last;
    }
    sum = (sum >> 16) + (sum & 0xFFFF);
    sum += (sum >> 16);
    return (uint16_t)(~sum);
}