#include <cmath>
#include "fmt/core.h"
#include "plog/Log.h"

#include "utils/macros.hpp"
#include "screendevice.hpp"

void ScreenDevice::moveTo(std::uint16_t x, std::uint16_t y) const {
	LOGI << fmt::format("\x1b[{};{}H", x, y);
}

std::uint16_t ScreenDevice::getUint16(std::uint16_t) const {
	return 0;
}

std::uint8_t ScreenDevice::getUint8(std::uint16_t) const {
	return 0;
}

void ScreenDevice::setUint16(std::uint16_t addr, std::uint16_t value) {
	const auto charValue = value & 0x00ff;

	const auto x = (addr % 16) + 1;
	const auto y = std::floor(addr / 16) + 1;
	moveTo(x * 2, y);
	LOGI << charValue;
}

void ScreenDevice::setUint8(std::uint16_t addr, std::uint8_t value) {
	UNUSED(addr);
	UNUSED(value);
}
