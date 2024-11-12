"Quite Quine"
"by Nate Westfall."
quote = "\x22"
var1 = "Quite Quine"
var2 = "by Nate Westfall."
var3 = [
    "var1",
    "Quite Quine"
    "var2"
    "by Nate Westfall."
    "",
    'print(var3[0])\nprint("\\""+var3[0]+"\\"")\nprint("\\""+var3[1]+"\\"")print(var3[1])',
]
print(var3[0])
print('"' + var3[0] + '"')
print('"' + var3[1] + '"')
print(var3[1])


"""

Quite Quine 
by Nate Westfall. 
was abc " Quite Quine ". was def " by Nate Westfall. ". pri abc. pri def. 
was abc " was \x61\x62\x63 \x22 abc \x22. was \x64\x65\x66 \x22 def \x22. pri \x61\x62\x63. pri \x64\x65\x66.\n was \x61\x62\x63 \x22 was ".
pri abc.





"""