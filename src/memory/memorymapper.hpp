#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <vector>

#include "IMemoryMapperDevice.hpp"

struct Region {
	std::shared_ptr<IMemoryMappedDevice> device;
	std::uint16_t start = 0;
	std::uint16_t end = 0;
	bool remap = false;
};

class MemoryMapper : public IMemoryMappedDevice {
private:
	std::vector<Region> regions;

public:
	std::optional<Region> findRegion(std::uint16_t addr) const;
	std::uint16_t getUint16(std::uint16_t addr) const override;
	std::uint8_t getUint8(std::uint16_t addr) const override;
	void setUint16(std::uint16_t addr, std::uint16_t value) override;
	void setUint8(std::uint16_t addr, std::uint8_t value) override;

	std::function<void()> map(const Region& region);
};
