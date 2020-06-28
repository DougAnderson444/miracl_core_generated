import os, glob
import shutil

rust_path = os.path.join('miracl_core', 'rust')
os.chdir(rust_path)

#curve_file_path = os.path.join('..', '..', 'curve_nos.txt')
os.system('python config64.py < ../../curve_nos.txt')

src_path = os.path.join('core', 'src');
dest_path = os.path.join('..', '..', 'rust_64', 'src')

for filename in glob.glob(os.path.join(src_path, '*.*')):
    if filename == 'lib.rs':
        continue
    shutil.copy(filename, dest_path)
# copytree(src_path, dest_path, ignore=ignore_patterns('lib.rs'))

os.system('git reset --hard')
shutil.rmtree('core', ignore_errors = False)