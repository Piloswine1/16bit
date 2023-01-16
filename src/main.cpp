#include <fmt/core.h>
#include <plog/Appenders/ColorConsoleAppender.h>
#include <plog/Formatters/MessageOnlyFormatter.h>
#include <plog/Init.h>
#include <plog/Log.h>
#include <plog/Severity.h>
#include <cstdint>
#include <memory>
#include <string>

#include "cpu/cpu.hpp"
#include "cpu/instructions.hpp"
#include "devices/screendevice.hpp"
#include "memory/memorymapper.hpp"

static plog::ColorConsoleAppender<plog::MessageOnlyFormatter>
	colorConsoleAppender;

int main() {
	plog::init(plog::info, &colorConsoleAppender);

	auto MM = std::make_unique<MemoryMapper>();

	const auto mem = std::make_shared<Memory>(256 * 256);
	auto writableMemory = mem->makeWritable();

	MM->map({mem, 0, 0xffff});
	MM->map({std::make_shared<ScreenDevice>(), 0x3000, 0x30ff, true});

	auto i = 0;

	auto cpu = CPU::CPU(std::move(MM));

	writableMemory[i++] = Instructions::MOV_LIT_REG;
	writableMemory[i++] = 0x00;
	writableMemory[i++] = static_cast<uint8_t>('H');
	writableMemory[i++] = CPU::R1;

	writableMemory[i++] = Instructions::MOV_REG_MEM;
	writableMemory[i++] = CPU::R1;
	writableMemory[i++] = 0x30;
	writableMemory[i++] = 0x00;

	writableMemory[i++] = Instructions::HLT;


	cpu.run();
	return 1;
}
