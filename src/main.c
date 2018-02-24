#include <stdint.h>
#include <unistd.h>
#include <stdio.h>

extern int32_t base64(char*);

char buf[7];

int main() {
  read(0, buf, 6);
  puts(base64(buf) == 1 ? "yes" : "no");
  return 0;
}
