from py_number import Number

n1 = Number(5)
n2 = Number(5)
print(n1)
print(n1 == n2)
print (Number(1 << 1337)) # no overflow

def hash_djb2(s: str):
    '''
    A version of Daniel J. Bernstein's djb2 string hashing algorithm
    Like many hashing algorithms, it relies on integer wrapping.
    '''

    n = Number(0)
    five = Number(5)

    for x in s:
        n = Number(ord(x)) + ((n << five) - n)
    return n

assert hash_djb2('l50_50') == Number(-1152549421)