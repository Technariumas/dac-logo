from __future__ import division
import numpy as np
from scipy import misc
#import matplotlib.pyplot as plt
import sys

#Usage: python make_image.py <input image> <output filename>

fname = sys.argv[1]
output = sys.argv[2]

im = misc.imread(fname, flatten=True, mode='L')

im[np.where(im > 0)] = 1
im = 1 - im

#plt.imshow(im, cmap='Greys')
#plt.show()

np.savetxt(output, np.transpose(np.nonzero(im)), fmt='%i', delimiter=",", newline="\n") 


