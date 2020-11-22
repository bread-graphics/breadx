# MIT/Apache2 License

import os
import platform
import shutil
import subprocess as sp

# If this is a Linux installation, install Xorg and the Xf86-dummy driver
if os.name == 'posix' and platform.system() != "Darwin":
  sp.run(["apt", "install", "xorg", "xserver-xorg-video-dummy"])

  shutil.copyfile("./ci/dummy_xorg_config.conf", "/etc/X11/xorg.conf")
