L3
===
[![test](https://github.com/smirzaei/l3/actions/workflows/test.yml/badge.svg)](https://github.com/smirzaei/l3/actions/workflows/test.yml)

An experimental request aware load balancer. This is useful if you are dealing with a custom TCP protocol. There is a catch though, both downstream (the clients) and upstream (servers) need to add the following header to every request/response, so that the load balancer knows the length of each message.

```
+-----+------+------+------+---------+---------+---------+---------+
| B0  |  B1  |  B2  |  B3  |   B4    |   B5    |   B6    |   B7    |
+-----+------+------+------+---------+---------+---------+---------+
| VER | RES1 | RES2 | RES3 | MSG_LEN | MSG_LEN | MSG_LEN | MSG_LEN |
+-----+------+------+------+---------+---------+---------+---------+

B0:   It's used for versioning, write 0x01.
B1:   A reserved byte, write 0x00.
B2:   A reserved byte, write 0x00.
B3:   A reserved byte, write 0x00.
B4-7: Message length. 32 bit unsigned integer, in little endian byte order.
```
