#!/usr/bin/env python3
"""Create a proper sized app icon with checkmark design"""
import struct
import zlib

def create_app_icon(size=128):
    """Create an icon with checkmark design at specified size"""
    width, height = size, size
    
    png_data = bytearray()
    png_data.extend(b'\x89PNG\r\n\x1a\n')
    
    # IHDR chunk - RGBA format
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    png_data.extend(create_chunk(b'IHDR', ihdr))
    
    # Create image data with blue-purple gradient and checkmark
    image_data = bytearray()
    
    for y in range(height):
        image_data.append(0)  # Filter type
        for x in range(width):
            # Calculate gradient color (blue to purple)
            ratio = x / width
            r = int(59 + (139 - 59) * ratio)
            g = int(130 + (92 - 130) * ratio)
            b = int(246 + (246 - 246) * ratio)
            
            # White circle background
            dx = x - width/2
            dy = y - height/2
            dist = (dx*dx + dy*dy) ** 0.5
            
            if dist < width * 0.35:  # Inside white circle
                # Check if we should draw checkmark
                is_checkmark = False
                
                # Normalized coordinates (0-1)
                nx = x / width
                ny = y / height
                
                # Checkmark left stroke
                if 0.3 <= nx <= 0.42 and 0.45 <= ny <= 0.7:
                    if abs((ny - 0.45) - (nx - 0.3) * 2.0) < 0.05:
                        is_checkmark = True
                
                # Checkmark right stroke
                if 0.42 <= nx <= 0.7 and 0.3 <= ny <= 0.58:
                    if abs((ny - 0.58) + (nx - 0.42) * 1.0) < 0.05:
                        is_checkmark = True
                
                if is_checkmark:
                    # Blue checkmark
                    image_data.extend([59, 130, 246, 255])
                else:
                    # White background
                    image_data.extend([255, 255, 255, 240])
            else:
                # Gradient background
                image_data.extend([r, g, b, 255])
    
    compressed = zlib.compress(bytes(image_data), 9)
    png_data.extend(create_chunk(b'IDAT', compressed))
    png_data.extend(create_chunk(b'IEND', b''))
    
    return bytes(png_data)

def create_chunk(chunk_type, data):
    """Create a PNG chunk"""
    chunk = bytearray()
    chunk.extend(struct.pack('>I', len(data)))
    chunk.extend(chunk_type)
    chunk.extend(data)
    crc = zlib.crc32(chunk_type + data) & 0xffffffff
    chunk.extend(struct.pack('>I', crc))
    return chunk

# Generate icon at 128x128
icon_data = create_app_icon(128)
with open('/Users/smile/work/silto/src-tauri/icons/icon.png', 'wb') as f:
    f.write(icon_data)

print('✅ App icon created successfully!')
print('   128x128 gradient background with white circle and blue checkmark')
