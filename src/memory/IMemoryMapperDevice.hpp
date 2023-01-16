#ifndef IMEMORY_MAPPER
#define IMEMORY_MAPPER

#include <cstdint>

class IMemoryMappedDevice {
public:
	virtual ~IMemoryMappedDevice() = default;
	virtual std::uint16_t getUint16(std::uint16_t addr) const = 0;
	virtual std::uint8_t getUint8(std::uint16_t addr) const = 0;
	virtual void setUint16(std::uint16_t addr, std::uint16_t value) = 0;
	virtual void setUint8(std::uint16_t addr, std::uint8_t value) = 0;
};

#endif  // IMEMORY_MAPPER
