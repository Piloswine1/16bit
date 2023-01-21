#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "cpu/cpu.hpp"
#include "memory/memorymapper.hpp"
#include "cpu/instructions.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

// TODO: somewhat make it
class MockScreenDevice

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"simple mapped test"_test = [] {
		auto MM = std::make_unique<MemoryMapper>();

		auto mem = std::make_unique<Memory>(256 * 256);
		auto writableMemory = mem->makeWritable();

		auto screenDevice = std::make_unique<ScreenDevice>();

		MM->map({std::move(mem), 0, 0xffff});
		MM->map({std::move(screenDevice), 0x3000, 0x30ff, true});

		auto i = 0;

		auto cpu = CPU::CPU(std::move(MM));

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0x12;
		writableMemory[i++] = 0x34;
		writableMemory[i++] = CPU::R1;

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0xAB;
		writableMemory[i++] = 0xCD;
		writableMemory[i++] = CPU::R2;

		writableMemory[i++] = Instructions::HLT;

		cpu.run();
	};
}
