#include "example.hpp"

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
