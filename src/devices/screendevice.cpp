#include <cmath>
#include <iostream>
#include "fmt/core.h"

#include "utils/macros.hpp"
#include "screendevice.hpp"

void ScreenDevice::moveTo(std::uint16_t x, std::uint16_t y) const {
	std::cout << fmt::format("\x1b[{};{}H", y, x);
}
void ScreenDevice::clear() const {
	std::cout << "\x1b[2J";
}
void ScreenDevice::setBold() const {
	std::cout << "\x1b[1m";
}
void ScreenDevice::setRegular() const {
	std::cout << "\x1b[0m";
}
void ScreenDevice::setDim() const {
	std::cout << "\x1b[2m";
}

std::uint16_t ScreenDevice::getUint16(std::uint16_t addr) const {
	UNUSED(addr);
	return 0;
}

std::uint8_t ScreenDevice::getUint8(std::uint16_t addr) const {
	UNUSED(addr);
	return 0;
}

void ScreenDevice::setUint16(std::uint16_t addr, std::uint16_t value) {
	const auto command = (value & 0xff00) >> 8;
	const auto charValue = value & 0x00ff;

	if (command == 0xff) {
		this->clear();
	} else if (command == 0x01) {
		this->setBold();
	} else if (command == 0x02) {
		this->setRegular();
	} else if (command == 0x03) {
		this->setDim();
	}

	const auto x = (addr % 16) + 1;
	const auto y = std::floor(addr / 16) + 1;
	this->moveTo(x * 2, y);
	std::cout << static_cast<char>(charValue);
}

void ScreenDevice::setUint8(std::uint16_t addr, std::uint8_t value) {
	UNUSED(addr);
	UNUSED(value);
}
