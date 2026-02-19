#pragma once
#include <string>
#include <unistd.h>
#include <sys/types.h>

class RandomAccessFile {
    int fd_;
public:
    explicit RandomAccessFile(const std::string &path, bool createIfMissing = true);
    ~RandomAccessFile();

    ssize_t readAt(void* buffer, size_t count, off_t offset);
    ssize_t writeAt(const void* buffer, size_t count, off_t offset);

    off_t size() const;
    void truncateTo(off_t newSize);
    void fsync();
};