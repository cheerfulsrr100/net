#include <iostream>

#include "Socket.h"
#include "lib.h"

int main(int argc, char const *argv[])
{
    int sockfd;
    auto socket = Socket::socket();
    InetAddr addr("127.0.0.1", 8080);

    socket.connect(addr);
    sockfd = socket.fd();
    lib::str_cli(stdin, sockfd);
    return 0;
}
