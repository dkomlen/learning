# Find six least signicant decimal digits of N-th Fibonacci number

def mul(m1, m2, k):
    return [[(m1[0][0]*m2[0][0]+m1[0][1]*m2[1][0]) % k,
             (m1[0][0]*m2[0][1]+m1[0][1]*m2[1][1]) % k],
            [(m1[1][0]*m2[0][0]+m1[1][1]*m2[1][0]) % k,
             (m1[1][0]*m2[0][1]+m1[1][1]*m2[1][1]) % k]]

def pow(x,n,k):
    ret = [[1,0],[0,1]]
    tmp = x

    if n == 0:
        return ret

    while n > 1:
        if n % 2:
            ret = mul(ret,tmp,k)
            n -= 1
        else:
            tmp = mul(tmp,tmp,k)
            n //= 2

    return mul(ret,tmp,k)

def fib(n,k):
    mat = pow([[0,1],[1,1]], n-1, k)
    return mat[1][1]

print(fib(100000001,10**6))

