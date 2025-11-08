#include "example.hpp"

#include <print>

auto main() -> int {

  // Test
  std::println("{}", smart::process_text_cpp("Library Test - C++"));
  std::println("{}", process_text("Library Test - Extern 'C'"));

  return 0;
}
