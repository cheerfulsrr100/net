#include "lib.h"

#include <unistd.h>
#include <cstring>
#include <iostream>

using std::cout;
using std::endl;

static int read_cnt;
static char* read_ptr;
static char read_buf[MAXLINE];

void lib::print_errno()
{
    cout << __func__ << " error " << strerror(errno) << endl;
}

void lib::str_echo(int sockfd)
{
    ssize_t n;
    char buf[MAXLINE];

again:
    while ((n = read(sockfd, buf, MAXLINE)) > 0) {
        write(sockfd, buf, n);
    }

    if (n < 0 && errno == EINTR) {
        goto again;
    }
    else if (n < 0) {
        cout << "str_echo: read error" << endl;
    }
}

void lib::str_cli(FILE* fp, int sockfd)
{
    char sendline[MAXLINE], recvline[MAXLINE];

    auto fgets = [=](char* ptr, int n, FILE* stream) {
        char* rptr;
        if ((rptr = ::fgets(ptr, n, stream)) == nullptr && ferror(stream) != 0) {
            print_errno();
        }
        return rptr;
    };

    while (fgets(sendline, MAXLINE, fp) != nullptr) {
        write(sockfd, sendline, strlen(sendline));

        if (readline(sockfd, recvline, MAXLINE) == 0) {
            cout << "str_cli: server terminated prematurely" << endl;
            exit(1);
        }

        if (fputs(recvline, stdout) == EOF) cout << "fputs error" << endl;
    }
}

ssize_t lib::read(int fd, void* buf, size_t count)
{
    ssize_t n;
    if ((n = ::read(fd, buf, count)) == -1) {
        print_errno();
    }
    return n;
}

static ssize_t my_read(int fd, char* ptr)
{
    if (read_cnt <= 0) {
    again:
        if ((read_cnt = ::read(fd, read_buf, sizeof(read_buf))) < 0) {
            if (errno == EINTR) {
                goto again;
            }
            return -1;
        }
        else if (read_cnt == 0) {
            return 0;
        }
        else {
            read_ptr = read_buf;
        }
    }

    read_cnt--;
    *ptr = *read_ptr++;
    return 1;
}

ssize_t lib::readline(int fd, void* vptr, size_t maxlen)
{
    ssize_t n, rc;
    char c, *ptr;

    ptr = static_cast<char*>(vptr);
    for (n = 1; n < maxlen; n++) {
        if ((rc = my_read(fd, &c)) == 1) {
            *ptr++ = c;
            if (c == '\n') break;
        }
        else if (rc == 0) {
            *ptr = 0;
            return n - 1;
        }
        else {
            cout << "readline error" << endl;
            return -1;
        }
    }
    *ptr = 0;
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
        // 减去已经写成功的数据量
        n_left -= n_writen;
        bufptr += n_writen;
    }

    return count;
}