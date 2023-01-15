#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>

#include "cpu.hpp"
#include "instructions.hpp"
#include "plog/Log.h"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	const auto mem = Memory(256);
	auto writableMemory = mem.makeWritable();

	auto cpu = CPU::CPU(mem);

	writableMemory[0] = Instructions::MOV_LIT_REG;
	writableMemory[1] = 0x12;
	writableMemory[2] = 0x34;
	writableMemory[3] = 0x2;

	writableMemory[4] = Instructions::MOV_LIT_REG;
	writableMemory[5] = 0xAB;
	writableMemory[6] = 0xCD;
	writableMemory[7] = 0x3;

	writableMemory[8] = Instructions::ADD_REG_REG;
	writableMemory[9] = 2;
	writableMemory[10] = 3;

	// LOGI << fmt::format("{}", writableMemory.buf());

	cpu.debug();

	cpu.step();
	cpu.debug();

	cpu.step();
	cpu.debug();

	cpu.step();
	cpu.debug();

	return 1;
}
