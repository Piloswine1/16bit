#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "memory/memory.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"uint8 operations"_test = [] {
		auto mem = Memory(255);

		mem.setUint8(0, 0xF);
		expect(mem.getUint8(0) == 15_i);

		mem.setUint8(1, 0xF);
		expect(mem.getUint8(0) == 15_i);
		expect(mem.getUint8(1) == 15_i);
	};

	"uint16 operations"_test = [] {
		auto mem = Memory(255);

		mem.setUint8(0, 0xF);
		mem.setUint8(1, 0xF);
		expect(mem.getUint16(0) == 3855_i);

		mem.setUint16(0, 0xFFFF);
		expect(mem.getUint8(0) == 255_i);
		expect(mem.getUint8(1) == 255_i);

		mem.setUint16(2, 0xF0F);
		expect(mem.getUint8(2) == 15_i);
		expect(mem.getUint8(3) == 15_i);
	};
};
