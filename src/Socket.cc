/***********************************************************
 *
 * @author Green
 * @email  alan_greens@outlook.com
 * @date   2022-08-08 15:53
 * @brief
 *
 **********************************************************/
#include "Socket.h"

#include <sys/socket.h>
#include <cerrno>
#include <cstring>
#include <iostream>

void Socket::bind(const InetAddr &addr)
{
    if (::bind(sockfd_, (const struct sockaddr *)addr.sockaddr(), sizeof(*addr.sockaddr())) == -1) {
        std::cout << "::bind error " << errno << std::endl;
    }
}

void Socket::listen()
{
    if (::listen(sockfd_, 1024) == -1) {
        std::cout << "::listen error " << errno << std::endl;
    }
}

int Socket::accept(InetAddr &addr)
{
    socklen_t addrlen;
    int accept_fd;
    if ((accept_fd = ::accept(sockfd_, (struct sockaddr *)addr.sockaddr(), &addrlen)) == -1) {
        std::cout << "::accept error " << strerror(errno) << std::endl;
    }
    return accept_fd;
}

void Socket::connect(const InetAddr &addr)
{
    if (::connect(sockfd_, (struct sockaddr *)addr.sockaddr(), sizeof(*addr.sockaddr())) == -1) {
        std::cout << "::connect error " << errno << std::endl;
    }
}

Socket Socket::socket()
{
    int fd = ::socket(AF_INET, SOCK_STREAM, 0);
    if (fd == -1) {
        std::cout << "::socket error " << errno << std::endl;
    }
    return Socket(fd);
}
