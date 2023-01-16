#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "cpu.hpp"
#include "instructions.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"basic cpu"_test = [] {
		const auto mem = Memory(10);
		auto cpu = CPU::CPU(mem);

		cpu.setRegister("r1", 12);
		expect(*cpu.getRegister("r1") == 12_i);

		cpu.setRegister("r2", 22);
		expect(*cpu.getRegister("r2") == 22_i);
	};

	"unknown cpu reg"_test = [] {
		const auto mem = Memory(10);
		auto cpu = CPU::CPU(mem);

		expect(nothrow([&] { cpu.setRegister("non_existent", 42); }));
		expect(!cpu.getRegister("non_existent"));
	};

	"mov lit reg"_test = [] {
		const auto mem = Memory(256);
		auto writableMemory = mem.makeWritable();
		auto cpu = CPU::CPU(mem);

		writableMemory[0] = Instructions::MOV_LIT_REG;
		writableMemory[1] = 0x12;
		writableMemory[2] = 0x34;
		writableMemory[3] = 0x2;

		cpu.step();

		expect(*cpu.getRegister("r1") == 4660_i);
	};

	"integrational sum"_test = [] {
		const auto mem = Memory(256);
		auto writableMemory = mem.makeWritable();

		auto i = 0;

		auto cpu = CPU::CPU(mem);

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0x12;
		writableMemory[i++] = 0x34;
		writableMemory[i++] = CPU::R1;

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0xAB;
		writableMemory[i++] = 0xCD;
		writableMemory[i++] = CPU::R2;

		writableMemory[i++] = Instructions::ADD_REG_REG;
		writableMemory[i++] = CPU::R1;
		writableMemory[i++] = CPU::R2;

		cpu.step();
		cpu.step();
		cpu.step();

		expect(*cpu.getRegister("acc") == 48641_i);
	};

	"jmp integrational sum"_test = [] {
		const auto mem = Memory(256 * 256);
		auto writableMemory = mem.makeWritable();

		// const auto IP = 0;
		auto i = 0;

		auto cpu = CPU::CPU(mem);

		writableMemory[i++] = Instructions::MOV_MEM_REG;
		writableMemory[i++] = 0x01;
		writableMemory[i++] = 0x00;
		writableMemory[i++] = CPU::R1;

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0x00;
		writableMemory[i++] = 0x01;
		writableMemory[i++] = CPU::R2;

		writableMemory[i++] = Instructions::ADD_REG_REG;
		writableMemory[i++] = CPU::R1;
		writableMemory[i++] = CPU::R2;

		writableMemory[i++] = Instructions::MOV_REG_MEM;
		writableMemory[i++] = CPU::ACC;
		writableMemory[i++] = 0x01;
		writableMemory[i++] = 0x00;
		// LOGI << fmt::format("{}", writableMemory.buf());

		writableMemory[i++] = Instructions::JMP_NOT_EQ;
		writableMemory[i++] = 0x00;
		writableMemory[i++] = 0x03;
		writableMemory[i++] = 0x00;
		writableMemory[i++] = 0x00;

		for (std::size_t i = 0; i < 4; ++i) {
			cpu.step();
			cpu.step();
			cpu.step();
			cpu.step();
		}

		expect(cpu.getMem16(0x0100) == 3_i);
	};

	"push/pop instructions"_test = [] {
		const auto mem = Memory(256 * 256);
		auto writableMemory = mem.makeWritable();

		auto i = 0;

		auto cpu = CPU::CPU(mem);

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0x51;
		writableMemory[i++] = 0x51;
		writableMemory[i++] = CPU::R1;

		writableMemory[i++] = Instructions::MOV_LIT_REG;
		writableMemory[i++] = 0x42;
		writableMemory[i++] = 0x42;
		writableMemory[i++] = CPU::R2;

		writableMemory[i++] = Instructions::PSH_REG;
		writableMemory[i++] = CPU::R1;

		writableMemory[i++] = Instructions::PSH_REG;
		writableMemory[i++] = CPU::R2;

		writableMemory[i++] = Instructions::POP;
		writableMemory[i++] = CPU::R3;

		writableMemory[i++] = Instructions::POP;
		writableMemory[i++] = CPU::R4;

		cpu.step();
		cpu.step();
		cpu.step();
		cpu.step();
		cpu.step();
		cpu.step();

		expect(*cpu.getRegister("r3") == 16962_i);
		expect(*cpu.getRegister("r4") == 20817_i);
	};
};
