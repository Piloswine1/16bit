#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "memory.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"uint8 operations"_test = [] {
		auto mem = Memory(255);

		mem.setUint8(0, 0xF);
		expect(mem.getUint8(0) == 15);

		mem.setUint8(1, 0xF);
		expect(mem.getUint8(0) == 15);
		expect(mem.getUint8(1) == 15);
	};

	"uint16 operations"_test = [] {
		auto mem = Memory(255);

		mem.setUint8(0, 0xF);
		mem.setUint8(1, 0xF);
		expect(mem.getUint16(0) == 0xF0F);

		mem.setUint16(0, 0xFFFF);
		expect(mem.getUint8(0) == 0xFF);
		expect(mem.getUint8(1) == 0xFF);

		mem.setUint16(2, 0xF0F);
		expect(mem.getUint8(2) == 0xF);
		expect(mem.getUint8(3) == 0xF);
	};
};
