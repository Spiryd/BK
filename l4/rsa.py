import random

def is_prime(n):
    if n <= 1 or (n % 2 == 0 and n > 2): 
        return False
    return all(n % i for i in range(3, int(n**0.5) + 1, 2))

def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a

def multiplicative_inverse(e, phi):
    d = 0
    x1 = 0
    x2 = 1
    y1 = 1
    temp_phi = phi
    
    while e > 0:
        temp1 = temp_phi//e
        temp2 = temp_phi - temp1 * e
        temp_phi = e
        e = temp2
        
        x = x2- temp1* x1
        x2 = x1
        x1 = x
        
        d = y1
        y1 = x
    
    if temp_phi == 1:
        return d + phi
    
def factorize(n, e, d):
    t = e*d - 1
    kphi = t
    while t % 2 == 0:
        t = t // 2
    a = 2
    while a < 1_000_000:
        k = t 
        while k < kphi:
            x = pow(a, k, n)
            if x != 1 and x != n-1 and pow(x, 2, n) == 1:
                r = gcd(x-1, n)
                return r, n//r
            k = k * 2
        a = a + 2

def generate_keypair(p, q):
    if not (is_prime(p) and is_prime(q)):
        raise ValueError('Both numbers must be prime.')
    elif p == q:
        raise ValueError('p and q cannot be equal')
    n = p * q
    phi = (p-1) * (q-1)
    e = random.randrange(1, phi)
    g = gcd(e, phi)
    while g != 1:
        e = random.randrange(1, phi)
        g = gcd(e, phi)
    d = multiplicative_inverse(e, phi)
    return ((n, e), (n, d))

p = 4858654271
q = 7678045183
print("p: ", p)
print("q: ", q)

publicA, privateA = generate_keypair(p, q)
publicB, privateB = generate_keypair(p, q)

print("PublicA: ", publicA)
print("PrivateA: ", privateA)
print("PublicB: ", publicB)

factored_p, factored_q = factorize(publicA[0], publicA[1], privateA[1])

print("Factored p: ", factored_p)
print("Factored q: ", factored_q)

d_B = multiplicative_inverse(publicB[1], (factored_p-1)*(factored_q-1))
print("Cracked PrivateB: ", (publicB[0], d_B))
print("PrivateB: ", privateB)