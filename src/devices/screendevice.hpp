#include <cstdint>

#include "memory/IMemoryMapperDevice.hpp"

class ScreenDevice : public IMemoryMappedDevice {
private:
	void moveTo(std::uint16_t x, std::uint16_t y) const;

public:
	~ScreenDevice() override = default;
	std::uint16_t getUint16(std::uint16_t addr) const override;
	std::uint8_t getUint8(std::uint16_t addr) const override;
	void setUint16(std::uint16_t addr, std::uint16_t value) override;
	void setUint8(std::uint16_t addr, std::uint8_t value) override;
};
