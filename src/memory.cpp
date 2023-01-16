#include <fmt/core.h>
#include <plog/Log.h>

#include "memory.hpp"

Memory::Memory(std::size_t size) {
	_buffer = std::make_shared<std::vector<std::uint8_t>>(size, 0);
}

std::size_t Memory::getByteLen() const {
	return _buffer->size();
}

std::uint8_t Memory::getUint8(std::size_t index) const {
	return (*_buffer)[index];
}

WritableMemory Memory::makeWritable() const {
	return {_buffer};
}

void Memory::setUint8(std::size_t index, std::uint8_t value) {
	LOGD << fmt::format("set [{}]: {}", index, value);
	(*_buffer)[index] = value;
}

std::uint16_t Memory::getUint16(std::size_t index) const {
	const std::uint16_t upper = (*_buffer)[index] << 8;
	const std::uint8_t lower = (*_buffer)[index + 1];
	LOGD << fmt::format("Upper: {}; Lower {}; res {}", upper, lower, upper | lower);
	return upper | lower;
}

void Memory::setUint16(std::size_t index, std::uint16_t value) {
	LOGD << fmt::format("set [{}]: {}", index, value >> 8);
	(*_buffer)[index] = value >> 8;
	LOGD << fmt::format("set [{}]: {}", index + 1, value & 0xFF);
	(*_buffer)[index + 1] = value & 0xFF;
}
