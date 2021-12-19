

import argparse
import getpass
import json
import os
import os.path
import sys

from . import loaders
from . import utils


defaultAdresses = {
    "abstract": "battilde",
    "unix": "battilde.socket",
    "inet": "localhost:9221",
    "inet4": "localhost:9221",
    "inet6": "localhost:9221"
}

def parse_args(argv):

    parser = argparse.ArgumentParser(
        description="The client to Battilde. Run this to connect to to the server.",
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('-n', '--name', help='Your player name (must be unique!). Defaults to username. All characters must be unicode letters, numbers or connection puctuation. The maximum size of a name is 256 bytes when encoded as utf8', default=None)
    parser.add_argument("-a", "--address", help="The address of the socket. When the socket type is 'abstract' this is just a name. When it is 'unix' this is a filename. When it is 'inet' is should be in the format 'address:port', eg 'localhost:8080'. Defaults depends on the socket type")
    parser.add_argument("-s", "--socket", help="the socket type. 'unix' is unix domain sockets, 'abstract' is abstract unix domain sockets and 'inet' is inet sockets. ", choices=["abstract", "unix", "inet", "inet4", "inet6"], default=("abstract" if sys.platform == "linux" else "inet"))
    parser.add_argument('-t', '--sprite', help="Player sprite. Format: <colourcode>-<letter>. Letter must be lowercase. The colourcode can be: 'r', 'g', 'b', 'c', 'y', 'm', any of the previous prefixed by 'l' or 'a'", default=None)
    parser.add_argument('-k', '--keybindings', help='The file with the keybinding configuration. This file is a JSON file.', default="default")
    parser.add_argument('-c', '--characters', help='The file with the character mappings for the graphics. If it is either of these names: {} it will be loaded from the charmaps directory.'.format(list(loaders.standardCharFiles.keys())), default="default")
    parser.add_argument('-o', '--logfile', help='All game messages will be written to this file.', default=None)
    parser.add_argument('--reset-style', help='Reset the style when it changes. Useful on some terminals', action="store_true")
    parser.add_argument('--blink-bright-background', help='Use blink attribute to make background brighter. Useful for terminals that don\'t have bright backgrounds normally. Implies --reset-style', action="store_true")
    parser.add_argument('-b', '--nocolours', '--nocolors', help='disable colours.', action="store_true")
    parser.add_argument('--ratuil-screen', help='The drawing backend that ratuil uses', choices=["curses", "ansi", "ansibuffered"], default="curses")
    
    args = parser.parse_args(argv)
    
    charmap = loaders.loadCharmap(args.characters)
    
    keybindings = loaders.loadKeybindings(args.keybindings)
    
    address = args.address
    if address is None:
        address = defaultAdresses[args.socket]
    if args.socket == "abstract":
        address = '\0' + address
    elif args.socket == "inet" or args.socket == "inet6" or args.socket == "inet4":
        hostname, _sep, port = address.rpartition(':')
        address = (hostname, int(port))
            
        
    
    colours = True
    if args.nocolours:
        colours = False
    
    name = args.name
    if name is None:
        username = getpass.getuser()
        name = username
    
    sprite = args.sprite
    if sprite is None:
        colour = utils.nick_colour(name)
        char = "x"
        letters = [c for c in name.upper() if c.isalpha()]
        if len(letters):
            char = letters[0]
        sprite = "{}-{}".format(colour, char)
    
    return (name, args.socket, address, keybindings, charmap, colours, args.logfile, args.ratuil_screen, {"always_reset": args.reset_style, "blink_bright_background": args.blink_bright_background}, sprite)
