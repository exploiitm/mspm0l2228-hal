import serial
import time
import subprocess

ser = serial.Serial('/dev/ttyACM0', timeout=10)
subprocess.Popen(["bash", "reset.sh"], stderr=subprocess.DEVNULL)
start_frame = bytes.fromhex("deadc0de")
buffer = b"\x00"*4

print("[*] Recieving startframe");
while True:
    buffer = (buffer + ser.read(1))[1:]
    if buffer == start_frame: break
    print("Debug:", buffer.hex())
print("[*] Recieved startframe");


from random import randint
while True:
    b = bytes([randint(0, 255)])
    ser.read(1)
    ser.write(b)
    assert ser.read(1) == b
    print("Passed")
