import serial
import time
import subprocess
from Crypto.Cipher import AES

ser = serial.Serial('/dev/ttyACM0', timeout=10)
subprocess.Popen(["bash", "reset.sh"], stderr=subprocess.DEVNULL)
start_frame = bytes.fromhex("deadc0de")
buffer = b"\x00"*4

print("[*] Recieving startframe");
while True:
    buffer = (buffer + ser.read(1))[1:]
    if buffer == start_frame: break
    # print("Debug:", buffer.hex())
print("[+] Recieved startframe");

key = ser.read(16)
print("[+] Recieved KEY")

for i in range(5):
    cipher = AES.new(key, AES.MODE_ECB)
    pt = ser.read(16)
    ct = ser.read(16)
    assert cipher.encrypt(pt) == ct, f"[!] Passed {i}, Failed {5-i}"
print("[+] Passed ECB Enc Tests [NoDma]")

for i in range(5):
    cipher = AES.new(key, AES.MODE_ECB)
    pt = ser.read(16)
    ct = ser.read(16)
    assert cipher.decrypt(pt) == ct, f"[!] Passed {i}, Failed {5-i}"
print("[+] Passed ECB Dec Tests [NoDma]")

for i in range(5):
    cipher = AES.new(key, AES.MODE_ECB)
    pt = ser.read(64)
    ct = ser.read(64)
    assert cipher.encrypt(pt) == ct, f"[!] Passed {i}, Failed {5-i}"
print("[+] Passed ECB Enc Tests [WithDma]")

for i in range(5):
    cipher = AES.new(key, AES.MODE_ECB)
    pt = ser.read(64)
    ct = ser.read(64)
    assert cipher.decrypt(pt) == ct, f"[!] Passed {i}, Failed {5-i}"
print("[+] Passed ECB Dec Tests [WithDma]")
