#include "cpu.hpp"
#include <algorithm>

namespace CPU {
CPU::CPU() : _memory(_registers.size() * 2) {
	_register_map.reserve(_registers.size());

	for (std::size_t i = 0; i < _registers.size(); i++) {
		_register_map.emplace(_registers[i], i * 2);
	}
}

std::optional<std::uint16_t> CPU::getRegister(const std::string_view& reg) {
	if (_register_map.contains(reg)) {
		return _memory.getUint16(_register_map[reg]);
	}
	return std::nullopt;
}

void CPU::setRegister(const std::string_view& reg, std::uint16_t var) {
	if (_register_map.contains(reg)) {
		_memory.setUint16(_register_map[reg], var);
	}
}
}  // namespace CPU
