#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <set>
#include <string_view>
#include <unordered_map>

#include "memory.hpp"

// TODO: make array reduction explicitly constexpr
// template <std::size_t N>
// constexpr auto make_register_map(std::array<std::string_view, N> array)
// -> decltype() {}

namespace CPU {
static const std::array<std::string_view, 10> _registers{
	"ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",
};

class CPU {
private:
	Memory _memory;
	std::unordered_map<std::string_view, int> _register_map;

public:
	CPU();
	std::optional<std::uint16_t> getRegister(const std::string_view& reg);
	void setRegister(const std::string_view& reg, std::uint16_t var);
};
}  // namespace CPU
