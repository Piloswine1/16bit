#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "cpu.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"write"_test = [] {
		const auto memory = Memory(256);
		auto writableMemory = memory.makeWritable();

		writableMemory[0] = 12;
		expect(writableMemory[0] == 12);

		writableMemory[0] = 1;
		expect(writableMemory[0] == 1);

		writableMemory[1] = 1;
		expect(writableMemory[1] == 1);

		writableMemory[2] = 1;
		expect(writableMemory[2] == 1);

		writableMemory[3] = 1;
		expect(writableMemory[3] == 1);
	};

	"wih cpu"_test = [] {
		const auto memory = Memory(256);
		auto writableMemory = memory.makeWritable();
		auto cpu = CPU::CPU(memory);

		writableMemory[0] = 12;
		expect(writableMemory[0] == 12);
		expect(cpu.getMem(0) == 12);

		writableMemory[0] = 1;
		expect(writableMemory[0] == 1);
		expect(cpu.getMem(0) == 1);

		writableMemory[1] = 1;
		expect(writableMemory[1] == 1);
		expect(cpu.getMem(1) == 1);
	};
}
