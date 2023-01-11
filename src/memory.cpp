#include <fmt/core.h>
#include <plog/Log.h>
#include <cstddef>
#include <cstdint>

#include "memory.hpp"

Memory::Memory(std::size_t size) : _buffer(size, 0) {}

std::uint8_t Memory::getUint8(std::size_t index) const {
	return _buffer[index];
}

void Memory::setUint8(std::size_t index, std::uint8_t value) {
	LOGD << fmt::format("set [{}]: {}", index, value);
	_buffer[index] = value;
}

std::uint16_t Memory::getUint16(std::size_t index) const {
	const std::uint16_t lower = _buffer[index] << 8;
	return lower | _buffer[index + 1];
}

void Memory::setUint16(std::size_t index, std::uint16_t value) {
	LOGD << fmt::format("set [{}]: {}", index, value >> 8);
	_buffer[index] = value >> 8;
	LOGD << fmt::format("set [{}]: {}", index + 1, value & 0xFF);
	_buffer[index + 1] = value & 0xFF;
}
