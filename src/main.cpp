#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <fmt/core.h>

#include "instructions.hpp"
#include "cpu.hpp"
#include "plog/Log.h"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	const auto mem = Memory(256);
	auto writableMemory = mem.makeWritable();

	auto cpu = CPU::CPU(mem);

	writableMemory[0] = Instructions::MOV_LIT_R1;
	writableMemory[1] = 0x12;
	writableMemory[2] = 0x34;

	writableMemory[3] = Instructions::MOV_LIT_R2;
	writableMemory[4] = 0xAB;
	writableMemory[5] = 0xCD;

	writableMemory[6] = Instructions::ADD_REG_REG;
	writableMemory[7] = 2;
	writableMemory[8] = 3;

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
