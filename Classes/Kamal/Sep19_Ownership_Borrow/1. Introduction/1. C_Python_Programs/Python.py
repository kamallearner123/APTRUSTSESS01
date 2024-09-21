import sys
s = ["Linux", "Windows", "Mac"]
t = s # Increases Ref Count
u = s # Increases Ref Count
print(sys.getrefcount(s)) # Prints 4

