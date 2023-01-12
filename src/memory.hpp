#include <cstddef>
#include <vector>
#include <cstdint>
#include <memory>
#include "writablememory.hpp"

class Memory {
private:
	std::shared_ptr<std::vector<std::uint8_t>> _buffer;

public:
	Memory(std::size_t size);

	WritableMemory makeWritable() const;

	std::uint8_t getUint8(std::size_t index) const;
	std::uint16_t getUint16(std::size_t index) const;
	void setUint8(std::size_t index, std::uint8_t value);
	void setUint16(std::size_t index, std::uint16_t value);
};
