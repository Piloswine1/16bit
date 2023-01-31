#include <algorithm>
#include <cstddef>
#include <numeric>

#include "cpu.hpp"
#include "fmt/core.h"
#include "instructions.hpp"
#include "plog/Log.h"

namespace CPU {
CPU::CPU(std::unique_ptr<IMemoryMappedDevice> mm)
	: _memory(std::move(mm)), _registers(global_registers.size() * 2) {
	_register_map.reserve(global_registers.size());

	for (std::size_t i = 0; i < global_registers.size(); i++) {
		_register_map.emplace(global_registers[i], i * 2);
		LOGD << fmt::format("[{}]: {}", global_registers[i], i * 2);
	}

	this->setRegister("sp", 0xffff - 1);
	this->setRegister("fp", 0xffff - 1);
}

std::uint16_t CPU::getMem16(std::size_t pos) const {
	return _memory->getUint16(pos);
}

std::uint8_t CPU::getMem(std::size_t pos) const {
	return _memory->getUint8(pos);
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
	const auto instruction = this->_memory->getUint8(*nextInstrcutionAddress);
	this->setRegister("ip", *nextInstrcutionAddress + 1);
	return instruction;
}

std::uint16_t CPU::fetch16() {
	const auto nextInstrcutionAddress = this->getRegister("ip");
	// FIXME: use optional
	if (!nextInstrcutionAddress)
		return 0;
	const auto instruction = this->_memory->getUint16(*nextInstrcutionAddress);
	this->setRegister("ip", *nextInstrcutionAddress + 2);
	return instruction;
}

void CPU::run() {
	auto halt = false;
	while (!halt) {
		halt = this->step();
	}
}

bool CPU::step() {
	const auto instruction = this->fetch();
	return this->execute(instruction);
}

void CPU::debug() {
	for (const auto& key : global_registers) {
		const auto regAddr = this->getRegister(key);
		LOGI << fmt::format("[{}]: {:#06x}", key, *regAddr);
	}
}

void CPU::push(std::uint16_t value) {
	const auto spAddr = this->getRegister("sp");
	this->_memory->setUint16(*spAddr, value);
	this->setRegister("sp", *spAddr - 2);
	this->_stackframe_size += 2;
}

std::uint16_t CPU::pop() {
	const auto nextSpAddr = *this->getRegister("sp") + 2;
	this->setRegister("sp", nextSpAddr);
	this->_stackframe_size -= 2;
	return this->_memory->getUint16(nextSpAddr);
}

std::uint16_t CPU::fetchRegisterIndex() {
	return (this->fetch() % global_registers.size()) * 2;
}

void CPU::pushState() {
	this->push(*this->getRegister("r1"));
	this->push(*this->getRegister("r2"));
	this->push(*this->getRegister("r3"));
	this->push(*this->getRegister("r4"));
	this->push(*this->getRegister("r5"));
	this->push(*this->getRegister("r6"));
	this->push(*this->getRegister("r7"));
	this->push(*this->getRegister("r8"));
	this->push(*this->getRegister("ip"));
	this->push(this->_stackframe_size + 2);

	this->setRegister("fp", *this->getRegister("sp"));
	this->_stackframe_size = 0;
}

void CPU::popState() {
	const auto framePtrAddr = *this->getRegister("fp");
	this->setRegister("sp", framePtrAddr);

	this->_stackframe_size = this->pop();
	const auto stackframe_size = this->_stackframe_size;

	this->setRegister("ip", this->pop());
	this->setRegister("r8", this->pop());
	this->setRegister("r7", this->pop());
	this->setRegister("r6", this->pop());
	this->setRegister("r5", this->pop());
	this->setRegister("r4", this->pop());
	this->setRegister("r3", this->pop());
	this->setRegister("r2", this->pop());
	this->setRegister("r1", this->pop());

	const auto nArgs = this->pop();
	for (std::size_t i = 0; i < nArgs; ++i) {
		this->pop();
	}

	this->setRegister("fp", framePtrAddr + stackframe_size);
}

void CPU::viewMemoryAt(std::uint16_t startPos, std::size_t n) {
	auto res = fmt::format("{:#06x}:", startPos);

	for (std::size_t it = startPos; it < startPos + 2 * n; it = it + 2) {
		const auto val = this->_memory->getUint16(it);
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

bool CPU::execute(std::uint16_t instruction) {
	switch (instruction) {
		// Move literal into reg
		case Instructions::MOV_LIT_REG: {
			const auto literal = this->fetch16();
			const auto reg = this->fetchRegisterIndex();
			this->_registers.setUint16(reg, literal);
			return false;
		}
		case Instructions::MOV_REG_REG: {
			const auto regFrom = this->fetchRegisterIndex();
			const auto regTo = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(regFrom);
			this->_registers.setUint16(regTo, value);
			return false;
		}
		// Move literal
		case Instructions::MOV_REG_MEM: {
			const auto regFrom = this->fetchRegisterIndex();
			const auto addr = this->fetch16();
			const auto value = this->_registers.getUint16(regFrom);
			this->_memory->setUint16(addr, value);
			return false;
		}
		// Move from memory to reg
		case Instructions::MOV_MEM_REG: {
			const auto addr = this->fetch16();
			const auto regTo = this->fetchRegisterIndex();
			const auto value = this->_memory->getUint16(addr);
			this->_registers.setUint16(regTo, value);
			return false;
		}
		case Instructions::MOV_LIT_MEM: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();
			this->_memory->setUint16(addr, value);
			return false;
		}
		// Move register* to register
		case Instructions::MOV_REG_PTR_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto ptr = this->_registers.getUint16(r1);
			const auto value = this->_memory->getUint16(ptr);
			this->_registers.setUint16(r2, value);
			return false;
		}
		// Move [lit + reg] to register
		case Instructions::MOV_LIT_OFF_REG: {
			const auto baseAddr = this->fetch16();
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto offset = this->_registers.getUint16(r1);

			const auto value = this->_memory->getUint16(baseAddr + offset);
			this->_registers.setUint16(r2, value);
			return false;
		}
		// Add registers
		case Instructions::ADD_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->setRegister("acc", regVal1 + regVal2);
			return false;
		}
		case Instructions::ADD_LIT_REG: {
			const auto lit = this->fetch16();
			const auto r1 = this->fetchRegisterIndex();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", lit + regVal);
			return false;
		}
		case Instructions::SUB_LIT_REG: {
			const auto lit = this->fetch16();
			const auto r1 = this->fetchRegisterIndex();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", regVal - lit);
			return false;
		}
		case Instructions::SUB_REG_LIT: {
			const auto lit = this->fetch16();
			const auto r1 = this->fetchRegisterIndex();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", lit - regVal);
			return false;
		}
		case Instructions::SUB_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->setRegister("acc", regVal1 - regVal2);
			return false;
		}
		case Instructions::MUL_LIT_REG: {
			const auto lit = this->fetch16();
			const auto r1 = this->fetchRegisterIndex();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", lit + regVal);
			return false;
		}
		case Instructions::MUL_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->setRegister("acc", regVal1 * regVal2);
			return false;
		}
		case Instructions::INC_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			this->_registers.setUint16(r1, value + 1);
			return false;
		}
		case Instructions::DEC_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			this->_registers.setUint16(r1, value - 1);
			return false;
		}
		case Instructions::LSF_REG_LIT: {
			const auto r1 = this->fetchRegisterIndex();
			const auto lit = this->fetch16();
			const auto regVal = this->_registers.getUint16(r1);
			this->_registers.setUint16(r1, regVal << lit);
			return false;
		}
		case Instructions::LSF_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->_registers.setUint16(r1, regVal1 << regVal2);
			return false;
		}
		case Instructions::RSF_REG_LIT: {
			const auto r1 = this->fetchRegisterIndex();
			const auto lit = this->fetch16();
			const auto regVal = this->_registers.getUint16(r1);
			this->_registers.setUint16(r1, regVal >> lit);
			return false;
		}
		case Instructions::RSF_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->_registers.setUint16(r1, regVal1 >> regVal2);
			return false;
		}
		case Instructions::AND_REG_LIT: {
			const auto r1 = this->fetchRegisterIndex();
			const auto lit = this->fetch16();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", regVal & lit);
			return false;
		}
		case Instructions::AND_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->setRegister("acc", regVal1 & regVal2);
			return false;
		}
		case Instructions::OR_REG_LIT: {
			const auto r1 = this->fetchRegisterIndex();
			const auto lit = this->fetch16();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", regVal | lit);
			return false;
		}
		case Instructions::OR_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->setRegister("acc", regVal1 | regVal2);
			return false;
		}
		case Instructions::XOR_REG_LIT: {
			const auto r1 = this->fetchRegisterIndex();
			const auto lit = this->fetch16();
			const auto regVal = this->_registers.getUint16(r1);
			this->setRegister("acc", regVal ^ lit);
			return false;
		}
		case Instructions::XOR_REG_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto r2 = this->fetchRegisterIndex();
			const auto regVal1 = this->_registers.getUint16(r1);
			const auto regVal2 = this->_registers.getUint16(r2);
			this->setRegister("acc", regVal1 ^ regVal2);
			return false;
		}
		case Instructions::NOT: {
			const auto r1 = this->fetchRegisterIndex();
			const auto regVal = this->_registers.getUint16(r1);
			this->_registers.setUint16(r1, (~regVal) & 0xffff);
			return false;
		}
		case Instructions::JMP_NOT_EQ: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value != this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JNE_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			const auto addr = this->fetch16();

			if (value != this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JEQ_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			const auto addr = this->fetch16();

			if (value == this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JEQ_LIT: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value == this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JLT_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			const auto addr = this->fetch16();

			if (value < this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JLT_LIT: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value < this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JGT_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			const auto addr = this->fetch16();

			if (value > this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JGT_LIT: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value > this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JLE_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			const auto addr = this->fetch16();

			if (value <= this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JLE_LIT: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value <= this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JGE_REG: {
			const auto r1 = this->fetchRegisterIndex();
			const auto value = this->_registers.getUint16(r1);
			const auto addr = this->fetch16();

			if (value >= this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::JGE_LIT: {
			const auto value = this->fetch16();
			const auto addr = this->fetch16();

			if (value >= this->getRegister("acc")) {
				this->setRegister("ip", addr);
			}

			return false;
		}
		case Instructions::PSH_LIT: {
			const auto value = this->fetch16();
			this->push(value);
			return false;
		}
		case Instructions::PSH_REG: {
			const auto regIndex = this->fetchRegisterIndex();
			this->push(this->_registers.getUint16(regIndex));
			return false;
		}
		case Instructions::POP: {
			const auto regIndex = this->fetchRegisterIndex();
			const auto value = this->pop();
			this->_registers.setUint16(regIndex, value);
			return false;
		}
		case Instructions::CALL_LIT: {
			const auto addr = this->fetch16();
			this->pushState();
			this->setRegister("ip", addr);
			return false;
		}
		case Instructions::CALL_REG: {
			const auto regIndex = this->fetchRegisterIndex();
			const auto addr = this->_registers.getUint16(regIndex);
			this->pushState();
			this->setRegister("ip", addr);
			return false;
		}
		case Instructions::RET: {
			this->popState();
			return false;
		}
		case Instructions::HLT: {
			return true;
		}
		default: {
			LOGF << fmt::format("Instruction {:#02x} not implemented", instruction);
		}
	}
	// TODO: somewhat return error
	return true;
}
}  // namespace CPU
