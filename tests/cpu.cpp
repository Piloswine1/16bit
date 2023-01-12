#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "instructions.hpp"
#include "cpu.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"basic cpu"_test = [] {
		const auto mem = Memory(10);
		auto cpu = CPU::CPU(mem);

		cpu.setRegister("r1", 12);
		expect(cpu.getRegister("r1") == 12);

		cpu.setRegister("r2", 22);
		expect(cpu.getRegister("r2") == 22);
	};

	"unknown cpu reg"_test = [] {
		const auto mem = Memory(10);
		auto cpu = CPU::CPU(mem);

		expect(nothrow([&] { cpu.setRegister("non_existent", 42); }));
		expect(!cpu.getRegister("non_existent"));
		expect(cpu.getRegister("non_existent") == std::nullopt);
	};

	"integrational sum"_test = [] {
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

		cpu.step();
		cpu.step();
		cpu.step();

		expect(cpu.getRegister("acc") == 0xbe01);
	};
};
