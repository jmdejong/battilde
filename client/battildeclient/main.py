#! /usr/bin/python3

import json
import sys
import signal
import getpass
import hashlib
import base64

from .connection import Connection
from .gameclient import Client
from .display import Display
from .parseargs import parse_args
from battildeclient.common import messages

def main(argv=None):
    
    (name, socketType, address, keybindings, characters, colours, logfile, ratuil_screen, ratuil_args, sprite) = parse_args(argv)
    
    if ratuil_screen == "ansibuffered":
        from ratuil.bufferedscreen import Screen
    elif ratuil_screen == "ansi":
        from ratuil.ansiscreen import Screen
    elif ratuil_screen == "curses":
        from ratuil.cursedscreen import Screen
    else:
        raise ValueError("Invalid ratuil screen selected")
    
    connection = Connection(socketType)
    try:
        connection.connect(address)
    except ConnectionRefusedError:
        print("ERROR: Could not connect to server.\nAre you sure that the server is running and that you're connecting to the right address?", file=sys.stderr)
        return
    
    if not introduce(connection, name, sprite):
        return
    error = None
    closeMessage = None
    
    #os.environ.setdefault("ESCDELAY", "25")
    screen = Screen(**ratuil_args)
    
    try:
        screen.initialize_terminal()

        display = Display(screen, characters)
        client = Client(display, name, connection, keybindings, logfile)
        signal.signal(signal.SIGWINCH, client.onSigwinch)
        try:
            client.start()
        except KeyboardInterrupt:
            client.close("^C caught, goodbye")
        except Exception as e:
            # throw the execption outside ncurses
            # so the cleanup can happen first
            error = e
        closeMessage = client.closeMessage
    finally:
        ## Set everything back to normal
        screen.finalize_terminal()
        
    
    if error is not None:
        raise error
    
    if closeMessage:
        print(closeMessage, file=sys.stderr)


def introduce(connection, name, sprite):
    connection.send(messages.IntroductionMessage(name, "player_"+sprite))
    print("introducing to server as {}".format(name))
    response = connection.receive()
    if response is None:
        print("connection lost")
        return False
    if isinstance(response, messages.ConnectedMessage):
        print("connection successful")
        return True
    if isinstance(response, messages.MessageMessage):
        return response.type == "connect"
    if isinstance(response, messages.ErrorMessage):
        print("Error: {}".format(response.to_json()), file=sys.stderr)
        return False
    
    print("Invalid server response: {}".format(response.to_json()), file=sys.stderr)
    return False
