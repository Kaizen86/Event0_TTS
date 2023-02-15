missing=["AO0","AW0","EY0","OY0","OY2","UH0","ZH"]
with open("src/vecs.txt") as file:
  lines = file.readlines()

for index,line in enumerate(lines):
  #print(line)
  block = False
  for item in missing:
    if item in line:
      lines[index] = "//"+line
      print(lines[index])

with open("src/vecs_fixed.txt","w") as file:
  file.writelines(lines)
