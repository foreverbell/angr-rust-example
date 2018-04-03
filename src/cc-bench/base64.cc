// C++ implementation, for benchmark.

#include <cstdint>
#include <string>
#include <vector>

using namespace std;

string base64_encode(const vector<uint8_t>& input) {
  const char bytes[] =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

  int len = input.size();
  int mod_len = len % 3;
  string output((len + 2) / 3 * 4, '=');
  int ptr = 0;

  for (int i = 0; i < len - mod_len; i += 3) {
    uint32_t first = input[i];
    uint32_t second = input[i + 1];
    uint32_t third = input[i + 2];

    uint32_t n = first << 16 | second << 8 | third;
    output[ptr++] = bytes[(n >> 18) & 63];
    output[ptr++] = bytes[(n >> 12) & 63];
    output[ptr++] = bytes[(n >> 6) & 63];
    output[ptr++] = bytes[(n >> 0) & 63];
  }

  if (mod_len == 1) {
    uint32_t n = input[len - 1] << 16;
    output[ptr++] = bytes[(n >> 18) & 63];
    output[ptr++] = bytes[(n >> 12) & 63];
  } else if (mod_len == 2) {
    uint32_t n = input[len - 2] << 16 | input[len - 1] << 8;
    output[ptr++] = bytes[(n >> 18) & 63];
    output[ptr++] = bytes[(n >> 12) & 63];
    output[ptr++] = bytes[(n >> 6) & 63];
  }
  return output;
}

string base64_encode(const string& str) {
  vector<uint8_t> input(str.length());

  for (int i = 0; i < str.length(); ++i) {
    input[i] = uint8_t(str[i]);
  }
  return base64_encode(input);
}

extern "C" {
  int32_t base64(char* str) {
    return base64_encode(str) == "aDFiYWJ5" ? 1 : 0;
  }
}
