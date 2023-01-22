#include <fmt/core.h>
#include <plog/Appenders/ConsoleAppender.h>
#include <plog/Init.h>
#include <boost/ut.hpp>

#include "cpu/cpu.hpp"
#include "cpu/instructions.hpp"
#include "memory/IMemoryMappedDevice.hpp"
#include "memory/memorymapper.hpp"

static plog::ConsoleAppender<plog::TxtFormatter> consoleAppender;

// TODO: somewhat make it
class MockScreenDevice final : public IMemoryMappedDevice {
public:
	inline static int count;

	std::uint16_t getUint16(std::uint16_t) const { return 0;}
	std::uint8_t getUint8(std::uint16_t) const { return 0; }
	void setUint16(std::uint16_t, std::uint16_t value) {
		if (value == 1) {
			count++;
		} else if ( value == 2) {
			count--;
		}
	}
	void setUint8(std::uint16_t, std::uint8_t) {}
};

int main() {
	plog::init(plog::debug, &consoleAppender);
	using namespace boost::ut;

	"simple mapped test"_test = [] {
		MockScreenDevice::count = 0;
		auto MM = std::make_unique<MemoryMapper>();

		auto mem = std::make_unique<Memory>(256 * 256);
		auto writableMemory = mem->makeWritable();

		auto screenDevice = std::make_unique<MockScreenDevice>();

		MM->map({std::move(mem), 0, 0xffff});
		MM->map({std::move(screenDevice), 0x3000, 0x30ff, true});

		auto i = 0;

		auto cpu = CPU::CPU(std::move(MM));

		writableMemory[i++] = Instructions::MOV_LIT_MEM;
		writableMemory[i++] = 0x00;
		writableMemory[i++] = 0x01;
		writableMemory[i++] = 0x30;
		writableMemory[i++] = 0x01;

		writableMemory[i++] = Instructions::MOV_LIT_MEM;
		writableMemory[i++] = 0x00;
		writableMemory[i++] = 0x02;
		writableMemory[i++] = 0x30;
		writableMemory[i++] = 0x01;

		writableMemory[i++] = Instructions::HLT;

		cpu.step();
		expect(MockScreenDevice::count == 1_i);
		
		cpu.step();
		expect(MockScreenDevice::count == 0_i);
	};
}
