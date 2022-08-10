#include "lib.h"

#include <unistd.h>
#include <cstring>
#include <iostream>

using std::cout;
using std::endl;

void lib::print_errno()
{
    cout << __func__ << " error " << strerror(errno) << endl;
}

void lib::str_echo(int sockfd)
{
    ssize_t n;
    char buf[MAXLINE];

    for (n = 0; n >= 0;) {
        n = read(sockfd, buf, MAXLINE);
        if (n > 0) {
            if (write(sockfd, buf, n) == -1) {
                print_errno();
            }
        }
        else if (n < 0 && n == EINTR) {
            n = 0;
            continue;
        }
        else {
            print_errno();
        }
    }
}

void lib::str_cli(FILE* fp, int sockfd) {}

ssize_t lib::read(int fd, void* buf, size_t count)
{
    ssize_t n;
    if ((n = ::read(fd, buf, count)) == -1) {
        print_errno();
    }
    return n;
}

ssize_t lib::write(int fd, const void* buf, size_t count)
{
    ssize_t n_left;
    ssize_t n_writen;
    const char* bufptr;

    bufptr = static_cast<const char*>(buf);

    for (n_left = count; n_left > 0;) {
        if ((n_writen = ::write(fd, bufptr, count)) <= 0) {
            // check EINTR signal
            if (n_writen == -1 && errno == EINTR) {
                n_writen = 0;
            }
            else {
                return -1;
            }
        };
        n_left -= n_writen;
        bufptr += n_writen;
    }

    return count;
}