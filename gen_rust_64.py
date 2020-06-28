## FIXME

import os, glob
import shutil

def copy_and_overwrite(from_path, to_path):
    if os.path.exists(to_path):
        shutil.rmtree(to_path)
    # shutil.copytree(from_path, to_path, ignore=shutil.ignore_patterns('lib.rs'))
    shutil.copytree(from_path, to_path)

rust_path = os.path.join('miracl_core', 'rust')
os.chdir(rust_path)

#curve_file_path = os.path.join('..', '..', 'curve_nos.txt')
os.system('python config64.py < ../../curve_nos.txt')

src_path = os.path.join('core', 'src');
dest_path = os.path.join('..', '..', 'rust_64', 'src')

# for filename in glob.glob(os.path.join(src_path, '*.*')):
#     if filename == 'core/src/lib.rs':
#         continue
#     shutil.copy(filename, dest_path)
# copytree(src_path, dest_path, ignore=ignore_patterns('lib.rs'))

shutil.move(os.path.join(dest_path, 'lib.rs'), os.path.join(dest_path, 'lib_backup.rs'))
copy_and_overwrite(src_path, dest_path)
shutil.move(os.path.join(dest_path, 'lib_backup.rs'), os.path.join(dest_path, 'lib.rs'))

os.system('git reset --hard')
shutil.rmtree('core', ignore_errors = True)
