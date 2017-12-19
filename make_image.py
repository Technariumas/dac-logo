from __future__ import division
import numpy as np
from scipy import misc
#import matplotlib.pyplot as plt
import sys

#Usage: python make_image.py <input image> <output filename>
#Input image should be 2-bit, i.e. black and white only.

fname = sys.argv[1]
output = sys.argv[2]

im = misc.imread(fname, flatten=True, mode='L')

#Comment this line if your image should not be inverted, i.e. if white regions should be bright.
im = 1 - im

#plt.imshow(im, cmap='Greys')
#plt.show()

np.savetxt(output, np.transpose(np.nonzero(im)), fmt='%i', delimiter=",", newline="\n") 


