

import os
import sys

import threading
from queue import Queue

import ratuil.inputs

from .inputhandler import InputHandler
from battildeclient.common import messages

class Client:
    
    def __init__(self, display, name, connection, keybindings, logFile=None):
        
        self.display = display
        self.name = name
        self.keepalive = True
        self.connection = connection
        self.logFile = logFile
        self.closeMessage = None
        self.helpVisible = False
        
        self.inputHandler = InputHandler(self, keybindings.actions)
        
        self.shortHelp = keybindings.shorthelp or ""
        self.longHelp = keybindings.longhelp or ""
        
        self.display.showInfo(self.shortHelp)
        self.display.setLongHelp(self.longHelp)
        self.queue = Queue()
        
    
    def sendMessage(self, message):
        self.connection.send(message)
    
    def sendInput(self, inp):
        message = messages.InputMessage(inp)
        self.sendMessage(message)
    
    def sendChat(self, text):
        try:
            self.sendMessage(messages.ChatMessage(text))
        except messages.InvalidMessageError as e:
            self.log(e.description)
    
    def start(self):
        threading.Thread(target=self.listen, daemon=True).start()
        threading.Thread(target=self.getInput, daemon=True).start()
        
        self.command_loop()
    
    def listen(self):
        try:
            self.connection.listen(self.pushMessage, self.onConnectionError)
        except BaseException as error:
            self.queue.put(("error", error))
    
    def pushMessage(self, message):
        self.queue.put(("message", message))
    
    def onConnectionError(self, error):
        self.queue.put(("error", error))
    
    def getInput(self):
        try:
            while True:
                key = self.display.screen.get_key()
                self.queue.put(("input", key))
        except BaseException as error:
            self.queue.put(("error", error))
    
    def close(self, msg=None):
        self.keepalive = False
        self.closeMessage = msg
    
    def toggleHelp(self):
        self.helpVisible = not self.helpVisible
        if self.helpVisible:
            for line in self.longHelp.splitlines():
                self.display.addMessage(line, "help")
            self.display.showHelp()
        else:
            self.display.hideHelp()
    
    
    def update(self, message):
        if message is None:
            self.close("Connection closed by server")
            return
        if isinstance(message, messages.ErrorMessage):
            error = message.errType
            if error == "nametaken":
                self.close("error: name is already taken")
                return
            if error == "invalidname":
                self.close("Invalid name error: "+ str(message.description))
                return
            self.log(message.errType + ": " + message.description)
        elif isinstance(message, messages.MessageMessage):
            self.log(message.text, message.type)
        elif isinstance(message, messages.WorldMessage):
            for msg in message.updates:
                self.handleWorldUpdate(msg)
    
    def handleWorldUpdate(self, msg):
        msgType = msg[0]
        if msgType == 'field':
            field = msg[1]
            fieldWidth = field['width']
            fieldHeight = field['height']
            self.display.resizeField((fieldWidth, fieldHeight))
            fieldCells = field['field']
            mapping = field['mapping']
            self.display.drawFieldCells(
                (
                    tuple(reversed(divmod(i, fieldWidth))),
                    mapping[spr]
                )
                for i, spr in enumerate(fieldCells))
        
        if msgType == 'changecells' and len(msg[1]):
            self.display.drawFieldCells(msg[1])
        
        if msgType == "playerpos":
            self.display.setFieldCenter(msg[1])
        
        if msgType == "health":
            health, maxHealth = msg[1]
            self.display.setHealth(health, maxHealth)
            if maxHealth is None:
                self.log("You have died. Restart the client to respawn")
        if msgType == "weapons":
            weapons, selected = msg[1]
            self.display.setWeapons(weapons, selected)
        if msgType == "ground":
            self.display.setGround(msg[1])
        if msgType == "message":
            text, type = msg[1:3]
            self.log(text, type)
        if msgType == "messages":
            for message in msg[1]:
                type = message[0]
                text = message[1]
                arg = None
                if len(message) > 2:
                    arg = message[2]
                if type == "options":
                    self.log(arg["description"])
                    for (command, description) in arg["options"]:
                        self.log("/q {:<24}   - {}".format(command, description))
                else:
                    self.log(text, type)
        if msgType == "options":
            if msg[1] != None:
                description, options = msg[1]
                self.log(description)
                for option in options:
                    self.log(option)
        
    
    def log(self, text, type=None):
        if not isinstance(text, str):
            text = str(text)
        self.display.addMessage(text, type)
        if self.logFile:
            with(open(self.logFile, 'a')) as f:
                f.write("[{}] {}\n".format(type or "", text))
    
    
    def command_loop(self):
        while self.keepalive:
            self.display.update()
            action = self.queue.get()
            if action[0] == "message":
                self.update(action[1])
            elif action[0] == "input":
                if action[1] == "^C":
                    raise KeyboardInterrupt
                self.inputHandler.onInput(action[1])
            elif action[0] == "error":
                raise action[1]
            elif action[0] == "sigwinch":
                self.display.update_size()
            else:
                raise Exception("invalid action in queue")
    
    def onSigwinch(self, signum, frame):
        self.queue.put(("sigwinch", (signum, frame)))
    



