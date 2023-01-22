#!/usr/bin/python3
import os
import shutil

print(os.getcwd())
print(os.listdir())
raws_folder = "unprocessed"

def copy(src_file, dest_path, dest_name):
  if type(dest_path) == str:
    dest_path = [dest_path]
  joinpath = '/'.join(dest_path)
  print(f"{src_file} --> {joinpath}/{dest_name}")
  os.makedirs(joinpath, exist_ok=True)
  shutil.copyfile(src_file, f"{joinpath}/{dest_name}")

for filename in os.listdir(raws_folder):
  filepath = f"{raws_folder}/{filename}"
  # skip directories (if any)
  if not os.path.isfile(filepath):
    continue
  speaker, *parts, output = filename.split("_")

  match len(parts):
    case 0:
      # 49
      copy(filepath, [speaker,"phonemes"], output)
    case 1:
      # 39: ['TH', 'thought2.ogg']
      copy(filepath, [speaker,"none",*parts], output)
    case 2:
      # 857: ['trust2', 'ER1', 'burned.ogg']
      copy(filepath, [speaker,*parts], output)
    case 3|4|5:
      # 374: ['anger1', 'vow', 'OW0', 'fellOW.ogg']
      copy(filepath, [speaker,*parts[:3]], output)
      """
    case 4:
      # 8: ['stress1', 'vow', 'AO0', 'was', 'Alread.ogg']
      pass
    case 5:
      # 4: ['trust1', 'vow', 'AO0', 'they', 'had', 'Al.ogg']
      pass
      """
    case other:
      raise Exception(f"Unhandled part length {other} (?!)")

print("Done")
