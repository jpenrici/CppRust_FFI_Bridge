#pragma once

#include <memory>
#include <string>

namespace smart {

/**
 * @brief The SmartString class
 * Class managing std::string using std::unique_ptr
 * It's non-copyable but movable - clear ownership semantics
 */
class SmartString {
public:
    // Explicit constructor: accept std::string_view without implicit conversions
    explicit SmartString(std::string_view value);

    // Delete copy constructor and copy assignment
    SmartString(const SmartString&) = delete;
    auto operator=(const SmartString&) -> SmartString = delete;

    // Allow move semantics (explicitly defaulted)
    SmartString(SmartString&&) noexcept = default;
    auto operator=(SmartString&&) noexcept -> SmartString& = default;

    // Destructor should also be noexcept
    ~SmartString() noexcept = default;

    // Return a copy of the underlying string
    [[nodiscard]] auto get() const noexcept -> std::string;

    // Append a suffix to the managed string
    void append_suffix(std::string_view suffix) noexcept;

private:
    std::unique_ptr<std::string> data_;
};

std::string process_text_cpp(const std::string& input);

} // namespace smart

// C ABI Wrapper (Extern "Rust")
extern "C" {

const char* process_text(const char* input);

void free_text(const char* ptr);

}
