#include <algorithm>
#include <cstddef>
#include <numeric>

#include "cpu.hpp"
#include "fmt/core.h"
#include "instructions.hpp"
#include "plog/Log.h"

namespace CPU {
CPU::CPU(Memory mem) : _memory(mem), _registers(global_registers.size() * 2) {
	_register_map.reserve(global_registers.size());

	for (std::size_t i = 0; i < global_registers.size(); i++) {
		_register_map.emplace(global_registers[i], i * 2);
		LOGD << fmt::format("[{}]: {}", global_registers[i], i * 2);
	}
}

std::uint8_t CPU::getMem(std::size_t pos) const {
	return _memory.getUint8(pos);
}

std::optional<std::uint16_t> CPU::getRegister(const std::string_view& reg) {
	if (_register_map.contains(reg)) {
		return _registers.getUint16(_register_map[reg]);
	}
	return std::nullopt;
}

void CPU::setRegister(const std::string_view& reg, std::uint16_t var) {
	if (_register_map.contains(reg)) {
		_registers.setUint16(_register_map[reg], var);
	}
}

std::uint8_t CPU::fetch() {
	const auto nextInstrcutionAddress = this->getRegister("ip");
	// FIXME: use optional
	if (!nextInstrcutionAddress)
		return 0;
	const auto instruction = this->_memory.getUint8(*nextInstrcutionAddress);
	this->setRegister("ip", *nextInstrcutionAddress + 1);
	return instruction;
}

std::uint16_t CPU::fetch16() {
	const auto nextInstrcutionAddress = this->getRegister("ip");
	// FIXME: use optional
	if (!nextInstrcutionAddress)
		return 0;
	const auto instruction = this->_memory.getUint16(*nextInstrcutionAddress);
	this->setRegister("ip", *nextInstrcutionAddress + 2);
	return instruction;
}

void CPU::step() {
	const auto instruction = this->fetch();
	this->execute(instruction);
}

void CPU::debug() {
	for (const auto& key : global_registers) {
		const auto regAddr = this->getRegister(key);
		LOGI << fmt::format("[{}]: {:#06x}", key, *regAddr);
	}
}

void CPU::viewMemoryAt(std::uint16_t startPos) {
	auto res = fmt::format("{:#06x}:", startPos);

	for (std::size_t it = startPos; it < startPos + 2 * 8; it = it + 2) {
		const auto val = this->_memory.getUint16(it);
		res += fmt::format(" {:#06x}", val);
	}

	LOGI << res;
	// TODO: Learn ranges in cpp
	// return fmt::format("{}: {}", startVal,
	// 				   std::views::iota(static_cast<int>(startPos + 1))
	// 				   | std::views::take(7)
	// 				   | std::views::transform([]() {})
	// 				   | std::views::join);
}

void CPU::execute(std::uint16_t instruction) {
	switch (instruction) {
		// Move literal into reg
		case Instructions::MOV_LIT_REG: {
			const auto literal = this->fetch16();
			const auto reg = (this->fetch() % global_registers.size()) * 2;
			this->_registers.setUint16(reg, literal);
			return;
		}
		case Instructions::MOV_REG_REG: {
			const auto regFrom = (this->fetch() % global_registers.size()) * 2;
			const auto regTo = (this->fetch() % global_registers.size()) * 2;
			const auto value = this->_registers.getUint16(regFrom);
			this->_registers.setUint16(regTo, value);
			return;
		}
		// Move literal
		case Instructions::MOV_REG_MEM: {
			const auto regFrom = (this->fetch() % global_registers.size()) * 2;
			const auto addr = this->fetch16();
			const auto value = this->_registers.getUint16(regFrom);
			this->_memory.setUint16(addr, value);
			return;
		}
		// Move from memory to reg
		case Instructions::MOV_MEM_REG: {
			const auto addr = this->fetch16();
			const auto regTo = (this->fetch() % global_registers.size()) * 2;
			const auto value = this->_memory.getUint16(addr);
			this->_registers.setUint16(regTo, value);
			return;
		}
		// Add registers
		case Instructions::ADD_REG_REG: {
			const auto r1 = this->fetch();
			const auto r2 = this->fetch();
			const auto regVal1 = this->_registers.getUint16(r1 * 2);
			const auto regVal2 = this->_registers.getUint16(r2 * 2);
			this->setRegister("acc", regVal1 + regVal2);
			return;
		}
		case Instructions::JMP_NOT_EQ: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value != this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return;
		}
	}
}
}  // namespace CPU
