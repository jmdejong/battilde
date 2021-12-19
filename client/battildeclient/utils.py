
import os

def clamp(val, lower, upper):
    """ val if it's between lower and upper, else the closest of the two"""
    return max(min(val, upper), lower)

def get(collection, i, default=None):
    """ Get an element in an indexed collection, or the default in case the index is out of bounds """
    if i < 0 or i >= len(collection):
        return default
    return collection[i]


def hash_djb2(name, num):
	for char in name:
		num ^= (num << 5) + (num >> 2) + ord(char)
	return num

def nick_colour(name):
    colours = ["c", "m", "g", "y", "lb", "a", "lc", "lm", "lg", "b"]
    seed = 5381
    num = hash_djb2(name, seed)
    colourid = num%len(colours)
    return colours[colourid]
