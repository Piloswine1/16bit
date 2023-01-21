#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "cpu/cpu.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"write"_test = [] {
		auto memory = std::make_unique<Memory>(256);
		auto writableMemory = memory->makeWritable();

		writableMemory[0] = 12;
		expect(writableMemory[0] == 12_i);

		writableMemory[0] = 1;
		expect(writableMemory[0] == 1_i);

		writableMemory[1] = 1;
		expect(writableMemory[1] == 1_i);

		writableMemory[2] = 1;
		expect(writableMemory[2] == 1_i);

		writableMemory[3] = 1;
		expect(writableMemory[3] == 1_i);
	};

	"wih cpu"_test = [] {
		auto memory = std::make_unique<Memory>(256);
		auto writableMemory = memory->makeWritable();
		auto cpu = CPU::CPU(std::move(memory));

		writableMemory[0] = 12;
		expect(writableMemory[0] == 12_i);
		expect(cpu.getMem(0) == 12_i);

		writableMemory[0] = 1;
		expect(writableMemory[0] == 1_i);
		expect(cpu.getMem(0) == 1_i);

		writableMemory[1] = 1;
		expect(writableMemory[1] == 1_i);
		expect(cpu.getMem(1) == 1_i);
	};
}
