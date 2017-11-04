def is_isogram(string):
    seen = 0
    norm_string = string.lower()
    for c in norm_string:
        if c in ('-', ' '):
            continue
        mask = 1 << ord(c)
        if seen & mask:
            return False
        seen |= mask
    return True
