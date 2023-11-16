import logging
import multiprocessing
import struct
import sys
import threading

import requests

import time
import socket
from threading import Thread
import random, datetime

logging.basicConfig(filename='../new.log', level=logging.DEBUG)
# 并发进程数
processNum = 1
# 单进程并发数
client_num = 1
# 发送时间间隔,秒
send_interval = 0.1

# 单次发送的帧数
send_num = 1000000

ADDRESS = [('0.0.0.0', 8020)]

lock = threading.Lock()


"""
    多线程模拟多个客户端，每个客户端初始化frame数据，每次初始化也就是每个线程对应的client具有不同的game_id，PVP地址当前为本地开启pvp服务地址。每个client多次传输所创建的frame数据，相同client每次传输的frame中Time标签值更新，其余不变
"""


def getmsTime():
    t = time.time()
    return int(round(t * 1000))


def getnsTime():
    t = time.time()
    return int(round(t * 1000 * 1000))


def client_conn(addr, tid):
    # 新建链接
    # ns = random.randint(0,10)
    # time.sleep(ns)
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, True)
    # 设置发送缓冲区大小为2048

    conn.setblocking(True)
    num = random.randint(0, len(ADDRESS) - 1)
    conn.connect(ADDRESS[num])

    dataAssemb = assemble_packet()

    for key in range(send_num):
        # 每个线程间隔多长时间发送一次frame数据
        time.sleep(float(send_interval))
        try:
            conn.sendall(dataAssemb)
        except Exception as e:
            print("send failed",e)

    conn.close()


def printInput(input):
    il = []
    for i in input:
        il.append(i)
    print(il)


def assemble_packet():
   msg = b'test data'

   num = len(msg)
   ret_msg = struct.pack("<ii", 1001,len(msg)) + msg

   return ret_msg


def Process(addr, pnum):
    tp = []
    for key in range(client_num):
        tempkey = pnum * client_num + key + 1
        print(tempkey)
        thread = Thread(target=client_conn, args=(addr, tempkey))
        tp.append(thread)

    for tr in tp:
        # time.sleep(2)
        tr.setDaemon(True)
        tr.start()

    for tr in tp:
        tr.join()

    print("end")


if __name__ == '__main__':

    tp = []
    for num in range(processNum):
        addnum = random.randint(0, len(ADDRESS) - 1)
        addr = ADDRESS[addnum]
        p = multiprocessing.Process(target=Process, args=(addr, num))
        p.start()
        tp.append(p)

    for p in tp:
        p.join()
        p.close()

    print("end")




