#include <sys/unistd.h>
#include <cstring>
#include <iostream>
#include "Socket.h"
#include "lib.h"

int main()
{

    int listenfd, connfd;
    pid_t childpid;

    auto sock = Socket::socket();
    InetAddr addr("127.0.0.1", 8080);

    sock.bind(addr);
    sock.listen();
    listenfd = sock.fd();

    while (true) {
        connfd   = sock.accept(addr);
        childpid = fork();
        if (childpid == 0) {
            close(listenfd);
            lib::str_echo(connfd);
            exit(0);
        }
        close(connfd);
    }

    return 0;
}
