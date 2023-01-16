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

	const auto subroutineAddr = 0x3000;
	auto i = 0;

	auto cpu = CPU::CPU(mem);

	const auto R1 = 2;
	const auto R4 = 5;
	const auto R8 = 9;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x33;
	writableMemory[i++] = 0x33;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x22;
	writableMemory[i++] = 0x22;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x11;
	writableMemory[i++] = 0x11;

	writableMemory[i++] = Instructions::MOV_LIT_REG;
	writableMemory[i++] = 0x12;
	writableMemory[i++] = 0x34;
	writableMemory[i++] = R1;

	writableMemory[i++] = Instructions::MOV_LIT_REG;
	writableMemory[i++] = 0x56;
	writableMemory[i++] = 0x78;
	writableMemory[i++] = R4;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x00;
	writableMemory[i++] = 0x00;

	writableMemory[i++] = Instructions::CALL_LIT;
	writableMemory[i++] = (subroutineAddr & 0xff00) >> 8;
	writableMemory[i++] = (subroutineAddr & 0x00ff);

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x04;
	writableMemory[i++] = 0x04;

	i = subroutineAddr;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x01;
	writableMemory[i++] = 0x02;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x03;
	writableMemory[i++] = 0x04;

	writableMemory[i++] = Instructions::PSH_LIT;
	writableMemory[i++] = 0x05;
	writableMemory[i++] = 0x06;

	writableMemory[i++] = Instructions::MOV_LIT_REG;
	writableMemory[i++] = 0x07;
	writableMemory[i++] = 0x08;
	writableMemory[i++] = R1;

	writableMemory[i++] = Instructions::MOV_LIT_REG;
	writableMemory[i++] = 0x09;
	writableMemory[i++] = 0x10;
	writableMemory[i++] = R8;

	writableMemory[i++] = Instructions::RET;

	cpu.debug();
	cpu.viewMemoryAt(*cpu.getRegister("ip"), 4);
	cpu.viewMemoryAt(0xffff - 1 - 42, 22);

	std::string prevCmd;
	for (std::string line; std::getline(std::cin, line);) {
		if (line.empty()) {
			line = prevCmd;
		}

		if (line.starts_with("n")) {
			cpu.step();
			cpu.debug();
			cpu.viewMemoryAt(*cpu.getRegister("ip"));
			cpu.viewMemoryAt(0xffff - 1 - 42, 22);
		}
		if (line.starts_with("q")) {
			break;
		}

		prevCmd = line;
	}

	return 1;
}
