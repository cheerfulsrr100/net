/***********************************************************
 *
 * @author Green
 * @email  alan_greens@outlook.com
 * @date   2022-08-08 15:54
 * @brief
 *
 **********************************************************/
#include "InetAddr.h"

#include <arpa/inet.h>
#include <cstring>
#include <iostream>

InetAddr::InetAddr(const std::string &ip, uint16_t port)
{

    memset(&addr_, 0, sizeof(addr_));
    addr_.sin_family = AF_INET;
    addr_.sin_port   = htobe16(port);

    if (inet_pton(AF_INET, ip.c_str(), &addr_.sin_addr) <= 0) {
        std::cout << "ip error" << std::endl;
    }
}
