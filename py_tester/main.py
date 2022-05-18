import librarytest as lt

print(lt.hellopy())

x = lt.Greeter()
print(x.greet("Papa"))

y = lt.Storage(3)
print(y.num)
y.num = 4
print(y.num)
#rint(y.num)
#print(y.half())

z = lt.Storage(2)
z.num = 6
print(y.sum_nums(z))

NN = lt.NeuralNetwork(None)