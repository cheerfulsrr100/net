/***********************************************************
 *
 * @author Green
 * @email  alan_greens@outlook.com
 * @date   2022-08-08 15:53
 * @brief
 *
 **********************************************************/
#ifndef UNTITLED_SOCKET_H
#define UNTITLED_SOCKET_H

#include <netinet/in.h>
#include "InetAddr.h"

class Socket {
public:
    explicit Socket(int fd) : sockfd_(fd) {}

    void bind(const InetAddr &addr);
    void listen();
    int accept(InetAddr &addr);
    void connect(const InetAddr &addr);

    static Socket socket();

    int fd() const { return sockfd_; }

private:
    const int sockfd_;
};

#endif  // UNTITLED_SOCKET_H
