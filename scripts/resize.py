import sys
from PIL import Image

# Check if a file name is provided as a command-line argument
if len(sys.argv) < 2:
    print("Usage: python resize.py <image_file_name>")
    sys.exit(1)

# Get the image file name from the command-line argument
image_file = sys.argv[1]
image_name = image_file.split('.')[0]  # Remove file extension

sizes = [32, 64, 128, 256, 512]
sizes2 = [(32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]

try:
    original_image = Image.open(image_file).convert("RGBA")

    icon_sizes = []
    for size in sizes:
        resized_image = original_image.resize((size, size), Image.LANCZOS)
        resized_image.save(f"{image_name}-{size}.png")
        icon_sizes.append(resized_image)

    # Uncomment the following line if you want to create an ICO file
    # icon_sizes[0].save(f"{image_name}.ico", format="ICO", sizes=sizes2)

except FileNotFoundError:
    print(f"Error: File '{image_file}' not found.")
    sys.exit(1)
except Exception as e:
    print(f"An error occurred: {e}")
    sys.exit(1)

print(f"Resized images saved successfully for {image_file}")
