#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <string>

#include "cpu.hpp"
#include "instructions.hpp"
#include "plog/Log.h"
#include "plog/Severity.h"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::info, &consoleAppender);
	const auto mem = Memory(256 * 256);
	auto writableMemory = mem.makeWritable();

	// const auto IP = 0;
	const auto ACC = 1;
	const auto R1 = 2;
	const auto R2 = 3;

	auto i = 0;

	auto cpu = CPU::CPU(mem);

	writableMemory[i++] = Instructions::MOV_MEM_REG;
	writableMemory[i++] = 0x01;
	writableMemory[i++] = 0x00;
	writableMemory[i++] = R1;

	writableMemory[i++] = Instructions::MOV_LIT_REG;
	writableMemory[i++] = 0x00;
	writableMemory[i++] = 0x01;
	writableMemory[i++] = R2;

	writableMemory[i++] = Instructions::ADD_REG_REG;
	writableMemory[i++] = R1;
	writableMemory[i++] = R2;

	writableMemory[i++] = Instructions::MOV_REG_MEM;
	writableMemory[i++] = ACC;
	writableMemory[i++] = 0x01;
	writableMemory[i++] = 0x00;
	// LOGI << fmt::format("{}", writableMemory.buf());

	writableMemory[i++] = Instructions::JMP_NOT_EQ;
	writableMemory[i++] = 0x00;
	writableMemory[i++] = 0x03;
	writableMemory[i++] = 0x00;
	writableMemory[i++] = 0x00;

	cpu.debug();
	cpu.viewMemoryAt(*cpu.getRegister("ip"));
	cpu.viewMemoryAt(0x0100);

	std::string prevCmd;
	for (std::string line; std::getline(std::cin, line);) {
		if (line.empty()) {
			line = prevCmd;
		}

		if (line.starts_with("n")) {
			cpu.step();
			cpu.debug();
			cpu.viewMemoryAt(*cpu.getRegister("ip"));
			cpu.viewMemoryAt(0x0100);
		}
		if (line.starts_with("q")) {
			break;
		}

		prevCmd = line;
	}

	return 1;
}
