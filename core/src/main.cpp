#include <fmt/core.h>
#include <plog/Appenders/ColorConsoleAppender.h>
#include <plog/Formatters/MessageOnlyFormatter.h>
#include <plog/Init.h>
#include <plog/Log.h>
#include <plog/Severity.h>
#include <argparse/argparse.hpp>
#include <filesystem>
#include <fstream>

#include "cpu/cpu.hpp"
#include "cpu/instructions.hpp"
#include "devices/screendevice.hpp"
#include "memory/memorymapper.hpp"
#include "src/utils/macros.hpp"

static plog::ColorConsoleAppender<plog::MessageOnlyFormatter>
	colorConsoleAppender;

static const auto MEM_SIZE = 256 * 256;

template <typename T>
std::optional<T> parse_to(const std::string_view& input) {
	int out;
	const std::from_chars_result result =
		std::from_chars(input.data(), input.data() + input.size(), out);
	if (result.ec == std::errc::invalid_argument ||
		result.ec == std::errc::result_out_of_range) {
		return std::nullopt;
	}
	return out;
}

void run_debug(CPU::CPU& cpu) {
	std::string prevCmd;
	for (std::string line; std::getline(std::cin, line);) {
		if (line.empty()) {
			line = prevCmd;
		}

		if (line.starts_with("n")) {
			const auto hlt = cpu.step();
			LOGD << "HLT instruction accured";
			if (hlt)
				return;

			cpu.debug();
			cpu.viewMemoryAt(*cpu.getRegister("ip"));
			cpu.viewMemoryAt(0x0000);
		}
		if (line.starts_with("r")) {
			cpu.run();
		}
		if (line.starts_with("d")) {
			cpu.debug();
		}
		if (line.starts_with("p")) {
			std::string_view view{line};
			view.remove_prefix(2);
			const auto pos = parse_to<uint16_t>(view);
			if (pos)
				cpu.viewMemoryAt(*pos);
		}
		if (line.starts_with("q")) {
			break;
		}

		prevCmd = line;
	}
}

int main(int argc, char** argv) {
	plog::init(plog::info, &colorConsoleAppender);
	argparse::ArgumentParser program("16bit");

	program.add_argument("file").help("mem file to execute");

	program.add_argument("-d")
		.help("runs vm in debug mode")
		.default_value(false)
		.implicit_value(true)
		.nargs(0);

	program.add_description("16 bit virtual machine");

	try {
		program.parse_args(argc, argv);
	} catch (const std::runtime_error& err) {
		std::cerr << err.what() << std::endl;
		std::cerr << program;
		std::exit(1);
	}

	LOGI << "Params: "
		 << fmt::format("\nfile: {}", program.get<std::string>("file"))
		 << fmt::format("\ndebug: {}", program.get<bool>("-d"));

	const auto file_name = program.get<std::string>("file");

	if (!std::filesystem::exists(file_name)) {
		LOGF << "File does not exists";
		return 1;
	}
	auto file = std::ifstream(file_name, std::ios::in | std::ios::binary);
	if (!file.is_open()) {
		LOGF << "Failed to open file";
		return 1;
	}

	file.seekg(0, std::ios::end);
	const auto fileSize = file.tellg();
	if (fileSize > MEM_SIZE) {
		LOGF << "Not enough mem to fit all instructions";
		return 1;
	}
	if (program.get<bool>("-d")) {
		LOGD << fmt::format("File size: {}", fileSize);
	}
	file.seekg(0, std::ios::beg);

	auto MM = std::make_unique<MemoryMapper>();

	auto mem = std::make_unique<Memory>(MEM_SIZE);
	auto writableMemory = mem->makeWritable();

	auto& buf = writableMemory.buf();
	buf.insert(buf.begin(), std::istream_iterator<uint8_t>(file),
			   std::istream_iterator<uint8_t>());

	auto screenDevice = std::make_unique<ScreenDevice>();

	MM->map({std::move(mem), 0, 0xffff});
	MM->map({std::move(screenDevice), 0x3000, 0x30ff, true});

	auto cpu = CPU::CPU(std::move(MM));

	if (program.get<bool>("-d")) {
		run_debug(cpu);
	} else {
		cpu.run();
	}

	return 1;
}
