#include "RandomAccessFile.hpp"
#include <fcntl.h>
#include <sys/stat.h>
#include <stdexcept>
#include <unistd.h>

RandomAccessFile::RandomAccessFile(const std::string &path, bool createIfMissing) {
    fd_ = open(path.c_str(), O_RDWR);
    if (fd_ < 0 && createIfMissing) {
        fd_ = open(path.c_str(), O_RDWR | O_CREAT, 0644);
    }
    if (fd_ < 0) throw std::runtime_error("open failed");
}

RandomAccessFile::~RandomAccessFile() {
    if (fd_ >= 0) close(fd_);
}

ssize_t RandomAccessFile::readAt(void* buffer, size_t count, off_t offset) {
    return pread(fd_, buffer, count, offset);
}

ssize_t RandomAccessFile::writeAt(const void* buffer, size_t count, off_t offset) {
    return pwrite(fd_, buffer, count, offset);
}

off_t RandomAccessFile::size() const {
    struct stat st;
    if (fstat(fd_, &st) != 0) return -1;
    return st.st_size;
}

void RandomAccessFile::truncateTo(off_t newSize) {
    if (ftruncate(fd_, newSize) != 0)
        throw std::runtime_error("ftruncate failed");
}

void RandomAccessFile::fsync() {
    if (::fsync(fd_) != 0)
        throw std::runtime_error("fsync failed");
}