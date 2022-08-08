/***********************************************************
 *
 * @author Green
 * @email  alan_greens@outlook.com
 * @date   2022-08-08 15:54
 * @brief  
 *
 **********************************************************/
#ifndef UNTITLED_INETADDR_H
#define UNTITLED_INETADDR_H

#include <string>
#include <netinet/in.h>


class InetAddr {

public:
    explicit InetAddr(const struct sockaddr_in &sockaddr) : addr_(sockaddr) {}

    InetAddr(const std::string &ip, uint16_t port);

    sa_family_t family() const { return addr_.sin_family; };
    const struct sockaddr_in *sockaddr() const { return &addr_; };


private:
    struct sockaddr_in addr_;

};


#endif //UNTITLED_INETADDR_H
