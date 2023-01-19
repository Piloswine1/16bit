#include <fmt/core.h>
#include <plog/Appenders/ColorConsoleAppender.h>
#include <plog/Formatters/MessageOnlyFormatter.h>
#include <plog/Init.h>
#include <plog/Log.h>
#include <plog/Severity.h>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>

#include "src/utils/macros.hpp"
#include "cpu/cpu.hpp"
#include "cpu/instructions.hpp"
#include "devices/screendevice.hpp"
#include "memory/memorymapper.hpp"

static plog::ColorConsoleAppender<plog::MessageOnlyFormatter>
	colorConsoleAppender;

int main() {
	plog::init(plog::info, &colorConsoleAppender);

	auto MM = std::make_unique<MemoryMapper>();

	auto mem = std::make_unique<Memory>(256 * 256);
	auto writableMemory = mem->makeWritable();

	auto screenDevice = std::make_unique<ScreenDevice>();

	MM->map({std::move(mem), 0, 0xffff});
	MM->map({std::move(screenDevice), 0x3000, 0x30ff, true});

	auto i = 0;

	auto cpu = CPU::CPU(std::move(MM));

	auto writeCharToScreen = [&](const auto& character, const auto& cmd, const auto& pos) {
		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = cmd;
		writableMemory[i++] = static_cast<uint8_t>(character);
		writableMemory[i++] = CPU::R1;

		writableMemory[i++] = Instructions::MOV_REG_MEM;
		writableMemory[i++] = CPU::R1;
		writableMemory[i++] = 0x30;
		writableMemory[i++] = static_cast<uint8_t>(pos);
	};

	writeCharToScreen(' ', 0xff, 0);

	//const auto msg = std::string("Vadik, Krasava!");
	for (size_t i = 0; i <= 0xff; ++i) {
		const auto cmd = i % 2 == 0 ? 0x03 : 0x02;
		writeCharToScreen('*', cmd, i);
	}

	writableMemory[i++] = Instructions::HLT;

	cpu.run();
	return 1;
}
