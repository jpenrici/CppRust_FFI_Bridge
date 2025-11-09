#include "example.hpp"

#include <cstring>
#include <iostream>
#include <string>

smart::SmartString::SmartString(std::string_view value)
    : data_(std::make_unique<std::string>(value)) {}

auto smart::SmartString::get() const noexcept -> std::string { return *data_; }

void smart::SmartString::append_suffix(std::string_view suffix) noexcept {
  *data_ += suffix;
}

auto smart::process_text_cpp(const std::string &input) -> std::string {
  SmartString s(input);
  s.append_suffix(" [C++ processed]");
  return s.get();
}

extern "C" {

const char *process_text(const char *input) {
  if (!input)
    return nullptr;

  std::string result = std::string(input) + " [C++ processed]";

  char *output = new char[result.size() + 1];
  std::memcpy(output, result.c_str(), result.size() + 1);

  std::cout << "[C++] process_text called with: " << input << std::endl;

  return output;
}

void free_text(const char *ptr) { delete[] ptr; }
}
