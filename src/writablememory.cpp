#include "writablememory.hpp"

WritableMemory::WritableMemory(
	std::shared_ptr<std::vector<std::uint8_t>> buffer)
	: _buffer(buffer) {}

std::uint8_t& WritableMemory::operator[](std::size_t pos) {
	return (*_buffer)[pos];
}

std::uint8_t& WritableMemory::operator[](std::size_t pos) const {
	return (*_buffer)[pos];
}

std::vector<std::uint8_t>& WritableMemory::buf() const {
	return *_buffer;
}
