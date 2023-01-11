#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "cpu.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"basic cpu"_test = [] {
		auto cpu = CPU::CPU();

		cpu.setRegister("r1", 12);
		expect(cpu.getRegister("r1") == 12);
	};

	"unknown cpu reg"_test = [] {
		auto cpu = CPU::CPU();

		expect(nothrow([&] { cpu.setRegister("non_existent", 42); }));
		expect(!cpu.getRegister("non_existent"));
		expect(cpu.getRegister("non_existent") == std::nullopt);
	};
};
