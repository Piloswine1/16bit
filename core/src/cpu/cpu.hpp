#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <set>
#include <string_view>
#include <unordered_map>
#include <vector>

#include "memory/memory.hpp"

// TODO: make array reduction explicitly constexpr
// template <std::size_t N>
// constexpr auto make_register_map(std::array<std::string_view, N> array)
// -> decltype() {}

namespace CPU {
static const std::array<std::string_view, 12> global_registers{
	"ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",
	"sp", "fp"
};

static const auto ACC = 1;
static const auto R1 = 2;
static const auto R2 = 3;
static const auto R3 = 4;
static const auto R4 = 5;
static const auto R5 = 6;
static const auto R6 = 7;
static const auto R7 = 8;
static const auto R8 = 9;

class CPU {
private:
	std::unique_ptr<IMemoryMappedDevice> _memory;
	Memory _registers;
	std::size_t _stackframe_size = 0;
	std::unordered_map<std::string_view, int> _register_map;

public:
	CPU(std::unique_ptr<IMemoryMappedDevice> mm);

	std::uint16_t getMem16(std::size_t pos) const;
	std::uint8_t getMem(std::size_t pos) const;

	void push(std::uint16_t value);
	std::uint16_t pop();

	void pushState();
	void popState();

	// Regs
	std::optional<std::uint16_t> getRegister(const std::string_view& reg);
	void setRegister(const std::string_view& reg, std::uint16_t var);

	std::uint16_t fetchRegisterIndex();
	std::uint8_t fetch();
	std::uint16_t fetch16();

	bool step();
	void run();
	bool execute(std::uint16_t instruction);
	void debug();
	void viewMemoryAt(std::uint16_t addr, std::size_t n = 8);
};
}  // namespace CPU
