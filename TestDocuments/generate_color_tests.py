open("color_tests.txt","w").write("\n".join(["was mario " + i + "." for i in open("colors.txt","r").read().split("\n")]))
