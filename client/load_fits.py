# %%
import matplotlib.pyplot as plt
import astropy.io.fits as fits
from PIL import Image
# %%
data = fits.open('received.fits')
img = Image.fromarray(data[0].data)

plt.imshow(img)
plt.show()