#include <algorithm>
#include <cstddef>

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
		LOGI << fmt::format("[{}]: {:#04x}", key, *regAddr);
	}
}

void CPU::execute(std::uint16_t instruction) {
	switch (instruction) {
		// Move literal
		case Instructions::MOV_LIT_R1: {
			const auto literal = this->fetch16();
			this->setRegister("r1", literal);
			return;
		}
		// Move literal
		case Instructions::MOV_LIT_R2: {
			const auto literal = this->fetch16();
			this->setRegister("r2", literal);
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
	}
}
}  // namespace CPU
