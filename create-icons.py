#!/usr/bin/env python3
"""
Create two icons:
1. App icon: 512x512 with colorful gradient and design
2. Menu bar icon: 22x22 simple black silhouette for visibility
"""
import struct
import zlib

def create_chunk(chunk_type, data):
    """Create a PNG chunk"""
    chunk = bytearray()
    chunk.extend(struct.pack('>I', len(data)))
    chunk.extend(chunk_type)
    chunk.extend(data)
    crc = zlib.crc32(chunk_type + data) & 0xffffffff
    chunk.extend(struct.pack('>I', crc))
    return chunk

def create_app_icon():
    """Create a beautiful 512x512 app icon with gradient and design"""
    width, height = 512, 512
    
    png_data = bytearray()
    png_data.extend(b'\x89PNG\r\n\x1a\n')
    
    # IHDR chunk - RGBA format
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    png_data.extend(create_chunk(b'IHDR', ihdr))
    
    image_data = bytearray()
    
    for y in range(height):
        image_data.append(0)  # Filter type
        for x in range(width):
            # Blue to purple gradient
            ratio_x = x / width
            ratio_y = y / height
            
            # Diagonal gradient
            ratio = (ratio_x + ratio_y) / 2
            r = int(59 + (139 - 59) * ratio)
            g = int(130 + (92 - 130) * ratio)
            b = int(246 + (246 - 246) * ratio)
            
            # Add rounded square mask
            margin = 90
            corner_radius = 110
            
            # Check if in rounded rectangle
            in_rect = True
            if x < margin + corner_radius and y < margin + corner_radius:
                dx = (margin + corner_radius) - x
                dy = (margin + corner_radius) - y
                if dx*dx + dy*dy > corner_radius*corner_radius:
                    in_rect = False
            elif x > width - margin - corner_radius and y < margin + corner_radius:
                dx = x - (width - margin - corner_radius)
                dy = (margin + corner_radius) - y
                if dx*dx + dy*dy > corner_radius*corner_radius:
                    in_rect = False
            elif x < margin + corner_radius and y > height - margin - corner_radius:
                dx = (margin + corner_radius) - x
                dy = y - (height - margin - corner_radius)
                if dx*dx + dy*dy > corner_radius*corner_radius:
                    in_rect = False
            elif x > width - margin - corner_radius and y > height - margin - corner_radius:
                dx = x - (width - margin - corner_radius)
                dy = y - (height - margin - corner_radius)
                if dx*dx + dy*dy > corner_radius*corner_radius:
                    in_rect = False
            
            if not in_rect:
                # Transparent
                image_data.extend([0, 0, 0, 0])
                continue
            
            # White center circle
            cx, cy = width/2, height/2
            dx = x - cx
            dy = y - cy
            dist = (dx*dx + dy*dy) ** 0.5
            
            if dist < 160:
                # Inside white circle - draw todo icon
                nx = (x - cx) / 160
                ny = (y - cy) / 160
                
                is_icon = False
                
                # Checkmark (larger and bolder)
                # Left stroke
                if -0.4 <= nx <= -0.1 and -0.1 <= ny <= 0.5:
                    if abs((ny + 0.1) - (nx + 0.4) * 1.5) < 0.15:
                        is_icon = True
                
                # Right stroke
                if -0.1 <= nx <= 0.5 and -0.5 <= ny <= 0.1:
                    if abs((ny - 0.1) + (nx + 0.1) * 1.0) < 0.15:
                        is_icon = True
                
                # List lines
                line_spacing = 0.25
                for i in range(3):
                    line_y = -0.4 + i * line_spacing
                    if 0.15 <= nx <= 0.6 and abs(ny - line_y) < 0.08:
                        is_icon = True
                
                if is_icon:
                    # Blue icon
                    image_data.extend([59, 130, 246, 255])
                else:
                    # White background
                    image_data.extend([255, 255, 255, 250])
            else:
                # Gradient background
                image_data.extend([r, g, b, 255])
    
    compressed = zlib.compress(bytes(image_data), 9)
    png_data.extend(create_chunk(b'IDAT', compressed))
    png_data.extend(create_chunk(b'IEND', b''))
    
    return bytes(png_data)

def create_menubar_icon():
    """Create a clear, recognizable 22x22 menu bar icon"""
    width, height = 22, 22
    
    png_data = bytearray()
    png_data.extend(b'\x89PNG\r\n\x1a\n')
    
    # IHDR chunk - RGBA format
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    png_data.extend(create_chunk(b'IHDR', ihdr))
    
    image_data = bytearray()
    
    for y in range(height):
        image_data.append(0)  # Filter type
        for x in range(width):
            is_icon = False
            
            # Simple list icon with 3 lines and checkmarks
            # Line 1 (top)
            if 5 <= x <= 18 and 4 <= y <= 5:
                is_icon = True
            # Checkbox 1
            if 2 <= x <= 3 and 4 <= y <= 5:
                is_icon = True
            
            # Line 2 (middle)
            if 5 <= x <= 18 and 10 <= y <= 11:
                is_icon = True
            # Checkbox 2  
            if 2 <= x <= 3 and 10 <= y <= 11:
                is_icon = True
            
            # Line 3 (bottom)
            if 5 <= x <= 18 and 16 <= y <= 17:
                is_icon = True
            # Checkbox 3
            if 2 <= x <= 3 and 16 <= y <= 17:
                is_icon = True
            
            if is_icon:
                # Black for menu bar
                image_data.extend([0, 0, 0, 255])
            else:
                # Transparent
                image_data.extend([0, 0, 0, 0])
    
    compressed = zlib.compress(bytes(image_data), 9)
    png_data.extend(create_chunk(b'IDAT', compressed))
    png_data.extend(create_chunk(b'IEND', b''))
    
    return bytes(png_data)

# Create both icons
print('Creating app icon (512x512)...')
app_icon = create_app_icon()
with open('/Users/smile/work/silto/src-tauri/icons/icon.png', 'wb') as f:
    f.write(app_icon)
print('✅ App icon created: icon.png')

print('\nCreating menu bar icon (22x22)...')
menubar_icon = create_menubar_icon()
with open('/Users/smile/work/silto/src-tauri/icons/icon-menubar.png', 'wb') as f:
    f.write(menubar_icon)
print('✅ Menu bar icon created: icon-menubar.png')

print('\n🎨 Icons created successfully!')
print('   • icon.png: 512x512 colorful app icon')
print('   • icon-menubar.png: 22x22 black menu bar icon')
