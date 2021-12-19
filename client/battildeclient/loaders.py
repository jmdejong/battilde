
import os

from .paths import keybindingsPath, charmapPath
from .charmap import CharMap
import json


standardKeyFiles = {
    "default": os.path.join(keybindingsPath, "default.json"),
    "azerty": os.path.join(keybindingsPath, "azerty.json")
}

def loadKeybindings(name):
    fname = None
    if name in standardKeyFiles:
        fname = standardKeyFiles[name]
    else:
        fname = name
    with open(fname) as f:
        data = json.load(f)
    bindings = {}
    shorthelp = ""
    longhelp = ""
    for ftemplate in data.get("templates", []):
        if ftemplate.partition(os.sep)[0] in {".", ".."}:
            ftemplate = os.path.relpath(ftemplate, fname)
        template = loadKeybindings(ftemplate)
        bindings.update(template.actions or {})
        shorthelp = template.shorthelp or shorthelp
        longhelp = template.longhelp or longhelp
    bindings.update(data.get("actions", {}))
    shorthelp = data.get("shorthelp", shorthelp)
    longhelp = data.get("longhelp", longhelp)
    if isinstance(shorthelp, list):
        shorthelp = "\n".join(shorthelp)
    if isinstance(longhelp, list):
        longhelp = "\n".join(longhelp)
    return KeyBindings(bindings, shorthelp, longhelp)

class KeyBindings:
    def __init__(self, actions, shorthelp, longhelp):
        self.actions = actions
        self.shorthelp = shorthelp
        self.longhelp = longhelp


standardCharFiles = {name: os.path.join(charmapPath, file) for name, file in {
    "default": "fullwidth.json",
    "halfwidth": "halfwidth.json",
    "hw": "halfwidth.json",
    "fullwidth": "fullwidth.json",
    "fw": "fullwidth.json",
    "emoji": "emoji.json"
}.items()}

def loadCharmapJson(name):
    
    fname = None
    if name in standardCharFiles:
        fname = standardCharFiles[name]
    else:
        fname = name
    with open(fname) as f:
        data = json.load(f)
    
    templates = []
    for ftemplate in data.get("templates", []):
        if ftemplate.partition(os.sep)[0] in {".", ".."}:
            ftemplate = os.path.relpath(ftemplate, fname)
        templates.extend(loadCharmapJson(ftemplate))
    
    templates.append(data)
    return templates

def loadCharmap(name):
    
    templates = loadCharmapJson(name)
    charmap = CharMap()
    for template in templates:
        charmap.apply_json(template)
    
    if charmap.character_width == 2:
        charmap.make_wide()
    
    return charmap
