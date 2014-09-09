import socket


ADDRESS = '127.0.0.1'
PORT = 5747

while True:
    text = raw_input()
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((ADDRESS, PORT))

    s.send(text + '\n')
    s.close()
    # data = s.recv(BUFFER_SIZE)
