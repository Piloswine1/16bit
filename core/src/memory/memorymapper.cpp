#include "memorymapper.hpp"
#include <pstl/glue_algorithm_defs.h>
#include <algorithm>
#include <cstddef>
#include <memory>
#include <optional>

Region const* MemoryMapper::findRegion(std::uint16_t addr) const {
	auto found = std::find_if(
		this->regions.begin(), this->regions.end(),
		[&addr](const auto& r) { return addr >= r->start && addr <= r->end; });
	if (found == this->regions.cend())
		return nullptr;
	return found->get();
}

std::function<void()> MemoryMapper::map(Region&& region) {
	const auto begin = this->regions.begin();
	const auto it = this->regions.insert(
		begin, std::make_shared<Region>(std::move(region)));
	return [this, &it] { this->regions.erase(it); };
}

std::uint16_t MemoryMapper::getUint16(std::uint16_t addr) const {
	const auto region = this->findRegion(addr);
	const auto finalAddr = region->remap ? addr - region->start : addr;
	return region->device->getUint16(finalAddr);
}

std::uint8_t MemoryMapper::getUint8(std::uint16_t addr) const {
	const auto region = this->findRegion(addr);
	const auto finalAddr = region->remap ? addr - region->start : addr;
	return region->device->getUint8(finalAddr);
}

void MemoryMapper::setUint16(std::uint16_t addr, std::uint16_t value) {
	const auto region = this->findRegion(addr);
	const auto finalAddr = region->remap ? addr - region->start : addr;
	return region->device->setUint16(finalAddr, value);
}

void MemoryMapper::setUint8(std::uint16_t addr, std::uint8_t value) {
	const auto region = this->findRegion(addr);
	const auto finalAddr = region->remap ? addr - region->start : addr;
	return region->device->setUint8(finalAddr, value);
}
