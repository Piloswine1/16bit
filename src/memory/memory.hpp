#include <cstddef>
#include <vector>
#include <cstdint>
#include <memory>

#include "IMemoryMapperDevice.hpp"
#include "writablememory.hpp"

class Memory: public IMemoryMappedDevice {
private:
	std::shared_ptr<std::vector<std::uint8_t>> _buffer;

public:
	Memory(std::size_t size);

	WritableMemory makeWritable() const;

	std::size_t getByteLen() const;
	std::uint16_t getUint16(std::uint16_t addr) const override;
	std::uint8_t getUint8(std::uint16_t addr) const override;
	void setUint16(std::uint16_t addr, std::uint16_t value) override;
	void setUint8(std::uint16_t addr, std::uint8_t value) override;
};
