from os import listdir, path

for folder in sorted(listdir()):
  if path.isfile(folder):
   continue

  padding = " "*(3-len(folder))
  files = [f.replace(".ogg","") for f in sorted(listdir(folder))]
  files = str(files).replace("'","\"")
  print(f"    \"{folder}\"{padding} => vec!{files},")
