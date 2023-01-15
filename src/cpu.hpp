#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <set>
#include <string_view>
#include <unordered_map>
#include <vector>

#include "memory.hpp"

// TODO: make array reduction explicitly constexpr
// template <std::size_t N>
// constexpr auto make_register_map(std::array<std::string_view, N> array)
// -> decltype() {}

namespace CPU {
static const std::array<std::string_view, 10> global_registers{
	"ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",
};

class CPU {
private:
	Memory _memory;
	Memory _registers;
	std::unordered_map<std::string_view, int> _register_map;

public:
	CPU(Memory);

	std::uint8_t getMem(std::size_t pos) const;

	// Regs
	std::optional<std::uint16_t> getRegister(const std::string_view& reg);
	void setRegister(const std::string_view& reg, std::uint16_t var);

	std::uint8_t fetch();
	std::uint16_t fetch16();

	void step();
	void execute(std::uint16_t instruction);
	void debug();
	void viewMemoryAt(std::uint16_t addr);
};
}  // namespace CPU
