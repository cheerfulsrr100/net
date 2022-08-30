/***********************************************************
 *
 * @author Green
 * @email  alan_greens@outlook.com
 * @date   2022-08-10 11:48
 * @brief
 *
 **********************************************************/

#ifndef UNIX_RS_LIB_H
#define UNIX_RS_LIB_H

#include <stdio.h>

#define MAXLINE  4096
#define IO_ERROR -1

namespace lib
{
void print_errno();

void str_echo(int sockfd);
void str_cli(FILE *fp, int sockfd);

ssize_t read(int fd, void *buf, size_t count);
ssize_t readline(int fd, void *ptr, size_t maxlen);
ssize_t write(int fd, const void *buf, size_t count);

}  // namespace lib

#endif