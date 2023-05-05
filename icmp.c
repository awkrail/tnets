#include <stdint.h>
#include <stddef.h>

#include "util.h"
#include "ip.h"
#include "icmp.h"

struct icmp_hdr {
  uint8_t type;
  uint8_t code;
  uint16_t sum;
  uint32_t values;
};

struct icmp_echo {
  uint8_t type;
  uint8_t code;
  uint16_t sum;
  uint16_t id;
  uint16_t seq;
};

static char *
icmp_type_ntoa(uint8_t type)
{
  switch(type) {
  case ICMP_TYPE_ECHOREPLY:
    return "ECHOREPLY";
  case ICMP_TYPE_DEST_UNREACH:
    return "DEST_UNREACH";
  case ICMP_TYPE_SOURCE_QUENCH:
    return "SOURCE_QUENCH";
  case ICMP_TYPE_REDIRECT:
    return "REDIRECT";
  case ICMP_TYPE_ECHO:
    return "ECHO";
  case ICMP_TYPE_TIME_EXCEEDED:
    return "TIME_EXCEEDED";
  case ICMP_TYPE_PARAM_PROBLEM:
    return "PARAM_PROBLEM";
  case ICMP_TYPE_TIMESTAMP:
    return "TIMESTAMP";
  case ICMP_TYPE_TIMESTAMPREPLY:
    return "TIMESTAMPREPLY";
  case ICMP_TYPE_INFO_REQUEST:
    return "INFO_REQUEST";
  case ICMP_TYPE_INFO_REPLY:
    return "INFO_REPLY";
  default:
    return "OTHER";
  }
}

static void
icmp_dump(const uint8_t *data, size_t len)
{
  struct icmp_hdr *hdr;
  struct icmp_echo *echo;

  flockfile(stderr);
  hdr = (struct icmp_hdr *)data;
  fprintf(stderr, "     type: %u (%s)\n", hdr->type, icmp_type_ntoa(hdr->type));
  fprintf(stderr, "     code: %u\n", hdr->code);
  fprintf(stderr, "      sum: 0x%04x\n", ntoh16(hdr->sum));
  switch(hdr->type) {
  case ICMP_TYPE_ECHOREPLY:
  case ICMP_TYPE_ECHO:
    echo = (struct icmp_echo *)hdr;
    fprintf(stderr, "     id: %u\n", ntoh16(echo->id));
    fprintf(stderr, "    seq: %u\n", ntoh16(echo->seq));
    break;
  default:
    fprintf(stderr, " values: 0x%08x\n", ntoh32(hdr->values));
  }
#ifdef HEXDUMP
  hexdump(stderr, data, len);
#endif
  funlockfile(stderr);
}

void
icmp_input(const uint8_t *data, size_t len, 
           ip_addr_t src, ip_addr_t dst, struct ip_iface *iface)
{
  struct icmp_hdr *hdr;
  char addr1[IP_ADDR_STR_LEN];
  char addr2[IP_ADDR_STR_LEN];

  if(len < ICMP_HDR_SIZE) {
    errorf("header length is smaller than ICMP_HDR_SIZE, len=%u", len);
    return;
  }

  hdr = (struct icmp_hdr *)data;
  if(cksum16((uint16_t *)hdr, len, 0) != 0) {
    errorf("checksum error");
    return;
  }

  debugf("%s => %s, len=%zu", 
      ip_addr_ntop(src, addr1, sizeof(addr1)), 
      ip_addr_ntop(dst, addr2, sizeof(addr2)), len);
  icmp_dump(data, len);
}

int
icmp_init(void)
{
  if(ip_protocol_register(IP_PROTOCOL_ICMP, icmp_input) == -1) {
    errorf("net_protocol_register() failure");
    return -1;
  }

  return 0;
}