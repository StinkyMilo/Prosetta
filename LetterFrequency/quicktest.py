i=0
a=1
b=1
temp=None
while i < 100:
    print(a)
    temp=b
    b=a+b
    a=temp
    i=i+1
