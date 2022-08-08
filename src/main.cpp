#include <iostream>
#include <cstring>
#include <sys/unistd.h>
#include "Socket.h"

int main() {

    auto sock = Socket::socket();
    InetAddr addr("127.0.0.1", 28889);

    sock.bind(addr);
    sock.listen();
    sock.accept(addr);

    return 0;
}
