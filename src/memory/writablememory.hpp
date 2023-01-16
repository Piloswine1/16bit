#include <cstddef>
#include <memory>
#include <vector>

class WritableMemory {
private:
	std::shared_ptr<std::vector<std::uint8_t>> _buffer;

public:
	WritableMemory(std::shared_ptr<std::vector<std::uint8_t>> buffer);

	std::uint8_t& operator[](std::size_t pos);
	std::uint8_t& operator[](std::size_t pos) const;

	std::vector<std::uint8_t>& buf() const;
};
