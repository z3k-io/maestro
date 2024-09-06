from PIL import Image

imageName = "monker-icon";

# sizes = [32, 64, 128, 256, 512]
# original_image = Image.open(f"{imageName}.png")




from PIL import Image

sizes = [32, 64, 128, 256, 512]
sizes2 = [(32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
original_image = Image.open(f"{imageName}.png").convert("RGBA")

icon_sizes = []
for size in sizes:
    resized_image = original_image.resize((size, size), Image.LANCZOS)
    resized_image.save(f"{imageName}-{256}.png")

    # icon_sizes.append(resized_image)

# icon_sizes[0].save("icon.ico", format="ICO", sizes=sizes2)
