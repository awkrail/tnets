#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

#include "util.h"
#include "net.h"
#include "ip.h"

struct ip_hdr {
  uint8_t vhl;
  uint8_t tos;
  uint16_t total;
  uint16_t id;
  uint16_t offset;
  uint8_t ttl;
  uint16_t sum;
  ip_addr_t src;
  ip_addr_t dst;
  uint8_t options[];
};

const ip_addr_t IP_ADDR_ANY       = 0x00000000;
const ip_addr_t IP_ADDR_BROADCAST = 0xffffffff; 

int
ip_addr_pton(const char *p, ip_addr_t *n)
{
  char *sp, *ep;
  int idx;
  long ret;

  sp = (char *)p;
  for (idx = 0; idx < 4; idx++) {
    ret = strtol(sp, &ep, 10);
    if(ret < 0 || ret > 255)
      return -1;

    if(ep == sp)
      return -1;

    if((idx == 3 && *ep != '\0') || (idx != 3 && *ep != '.'))
      return -1;

    ((uint8_t *)n)[idx] = ret;
    sp = ep + 1;
  }
  return 0;
}

static void
ip_dump(const uint8_t *data, size_t len)
{
  struct ip_hdr *hdr;
  uint8_t v, hl, hlen;
  uint16_t total, offset;
  char addr[IP_ADDR_STR_LEN];

  flockfile(stderr);
  hdr = (struct ip_hdr *)data;
  v = (hdr->vhl & 0xf0) >> 4;
  hl = hdr->vhl & 0x0f;
  hlen = hl << 2;
  fprintf(stderr, "     vhl: 0x%02x [v:%u, hl: %u (%u)]\n", hdr->vhl, v, hl, hlen);
  fprintf(stderr, "     tos: 0x%02x\n", hdr->tos);
  total = ntoh16(hdr->total); /* multi-byte length should be converted to byte-order */
  fprintf(stderr, "   total: %u (payload: %u)\n", total, total - hlen);
  fprintf(stderr, "      id: %u\n", ntoh16(hdr->id));
  offset = ntoh16(hdr->offset);
  fprintf(stderr, "  offset: 0x%04x [flags=%x, offset=%u]\n", 
          offset, (offset & 0xe000) >> 13, offset & 0x1fff);
  fprintf(stderr, "     ttl: %u\n", hdr->ttl);
  fprintf(stderr, "protocol: %u\n", hdr->protocol);
  fprintf(stderr, "     sum: 0x%04x\n", ntoh16(hdr->sum));
  fprintf(stderr, "     src: %s\n", ip_addr_ntop(hdr->src, addr, sizeo(addr)));
  fprintf(stderr, "     dst: %s\n", ip_addr_ntop(hdr->dst, addr, sizeo(addr)));
#ifdef HEXDUMP
  hexdump(stderr, data, len);
#endif
  funlockfile(stderr);
}

static void
ip_input(const uint8_t *data, size_t len, struct net_device *dev)
{
  // TODO: implement ip_input
}

char *
ip_addr_ntop(ip_addr_t n, char *p, size_t size)
{
  uint8_t *u8;

  u8 = (uint8_t *)&n;
  snprintf(p, size, "%d.%d.%d.%d", u8[0], u8[1], u8[2], u8[3]);
  return p;
}

static void
ip_input(const uint8_t *data, size_t len, struct net_device *dev)
{
  debugf("dev=%s, len=%zu", dev->name, len);
  debugdump(data, len);
}

int
ip_init(void)
{
  if(net_protocol_register(NET_POROTOCOL_TYPE_IP, ip_input) == -1) {
    errorf("net_protocol_register() failure");
    return -1;
  }
  return 0;
}
