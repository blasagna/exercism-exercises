#
# Skeleton file for the Python "Bob" exercise.
#


def hey(what):
    what = what.strip()

    if len(what) == 0:
        reply = 'Fine. Be that way!'
    elif any(c.isupper() for c in what) and not any(c.islower() for c in what):
        reply = 'Whoa, chill out!'
    elif what[-1] == '?':
        reply = 'Sure.'
    else:
        reply = 'Whatever.'

    return reply
