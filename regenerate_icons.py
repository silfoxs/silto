#!/usr/bin/env python3
"""
Create premium icons directly in python to ensure valid RGBA PNG format.
Design: "Big Sur" style squircle, indigo-purple gradient, white 3D circle, blue checkmark + notes.
"""
import struct
import zlib
import math

def create_chunk(chunk_type, data):
    chunk = bytearray()
    chunk.extend(struct.pack('>I', len(data)))
    chunk.extend(chunk_type)
    chunk.extend(data)
    crc = zlib.crc32(chunk_type + data) & 0xffffffff
    chunk.extend(struct.pack('>I', crc))
    return chunk

def get_color(x, y, width, height):
    # Normalized coordinates
    nx = x / width
    ny = y / height
    
    # 1. Background Mask: Squircle
    # Standard macOS padding is often ~10-15% empty space if you want it to look "standard size" relative to others
    padding = width * 0.20  # 20% padding to make it visually smaller
    radius = (width - 2 * padding) * 0.223 
    
    # Center
    cx, cy = width/2, height/2
    # Half extents of the drawn box
    hx, hy = width/2 - padding, height/2 - padding
    
    # Absolute pos relative to center
    ax = abs(x - cx)
    ay = abs(y - cy)
    
    # Distance to rounded rect edge
    dx = max(ax - (hx - radius), 0)
    dy = max(ay - (hy - radius), 0)
    dist = (dx*dx + dy*dy)**0.5
    
    alpha = 0
    if dist <= radius:
        if dist > radius - 1:
            alpha = (radius - dist)
        else:
            alpha = 1.0
            
    if alpha <= 0:
        return (0, 0, 0, 0)

    # 2. Gradient Background (Deep Indigo to Purple)
    # 45 degree angle roughly
    grad_t = (nx + ny) / 2
    # Start: #4f46e5 (Indigo 600) -> End: #9333ea (Purple 600)
    # A bit darker/richer for minimalist look
    r1, g1, b1 = 79, 70, 229
    r2, g2, b2 = 147, 51, 234
    
    r = int(r1 + (r2 - r1) * grad_t)
    g = int(g1 + (g2 - g1) * grad_t)
    b = int(b1 + (b2 - b1) * grad_t)
    
    final_r, final_g, final_b, final_a = r, g, b, int(255 * alpha)

    # 3. Minimalist Checkmark (White)
    # Center region for glyph
    glyph_size = (width - 2 * padding) * 0.55
    gx = (x - cx) / glyph_size
    gy = (y - cy) / glyph_size
    
    # Checkmark strokes
    # Stroke width
    stroke = 0.12
    
    # Points (normalized centered coordinates)
    # P1(-0.35, 0.05) -> P2(-0.05, 0.35) -> P3(0.45, -0.45)
    
    def dist_segment(px, py, x1, y1, x2, y2):
        l2 = (x1-x2)**2 + (y1-y2)**2
        if l2 == 0: return ((px-x1)**2 + (py-y1)**2)**0.5
        t = ((px-x1)*(x2-x1) + (py-y1)*(y2-y1)) / l2
        t = max(0, min(1, t))
        return ((px - (x1 + t*(x2-x1)))**2 + (py - (y1 + t*(y2-y1)))**2)**0.5

    d1 = dist_segment(gx, gy, -0.35, 0.05, -0.05, 0.35) # Down stroke
    d2 = dist_segment(gx, gy, -0.05, 0.35, 0.45, -0.45) # Up stroke
    
    check_dist = min(d1, d2)
    
    if check_dist < stroke:
        check_a = 1.0
        if check_dist > stroke - 0.01:
            check_a = (stroke - check_dist) / 0.01
            
        # Composite white on top
        final_r = int(final_r * (1-check_a) + 255 * check_a)
        final_g = int(final_g * (1-check_a) + 255 * check_a)
        final_b = int(final_b * (1-check_a) + 255 * check_a)
        
    return (final_r, final_g, final_b, final_a)

def generate_png(filename, width, height):
    # Prepare PNG data
    png_data = bytearray()
    png_data.extend(b'\x89PNG\r\n\x1a\n')
    
    # IHDR: 8 bit depth, 6=Truecolor+Alpha
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    png_data.extend(create_chunk(b'IHDR', ihdr))
    
    # IDAT: Raw RGBA data
    raw_data = bytearray()
    for y in range(height):
        raw_data.append(0) # Filter type 0 (None)
        for x in range(width):
            r, g, b, a = get_color(x, y, width, height)
            raw_data.extend([r, g, b, a])
            
    compressed = zlib.compress(bytes(raw_data), 9)
    png_data.extend(create_chunk(b'IDAT', compressed))
    png_data.extend(create_chunk(b'IEND', b''))
    
    with open(filename, 'wb') as f:
        f.write(png_data)
    print(f"Generated {filename}")


def generate_menubar_icon(filename):
    # macOS Menu Bar Icon: 22x22
    # Style: Template (Pure Black + Alpha)
    # Content: Thick Checkmark
    width, height = 22, 22
    
    png_data = bytearray()
    png_data.extend(b'\x89PNG\r\n\x1a\n')
    
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    png_data.extend(create_chunk(b'IHDR', ihdr))
    
    raw_data = bytearray()
    for y in range(height):
        raw_data.append(0)
        for x in range(width):
            # Coordinates normalized
            nx = x / width
            ny = y / height
            
            # Draw Checkmark
            # Center it
            cx, cy = 0.5, 0.5
            
            # Simple line SDF
            def dist_segment(px, py, x1, y1, x2, y2):
                l2 = (x1-x2)**2 + (y1-y2)**2
                if l2 == 0: return ((px-x1)**2 + (py-y1)**2)**0.5
                t = ((px-x1)*(x2-x1) + (py-y1)*(y2-y1)) / l2
                t = max(0, min(1, t))
                return ((px - (x1 + t*(x2-x1)))**2 + (py - (y1 + t*(y2-y1)))**2)**0.5
            
            # Checkmark shape - tweaked for small 22px grid
            # P1(0.2, 0.5) -> P2(0.45, 0.75) -> P3(0.85, 0.25)
            d1 = dist_segment(nx, ny, 0.2, 0.5, 0.45, 0.75)
            d2 = dist_segment(nx, ny, 0.45, 0.75, 0.85, 0.25)
            
            dist = min(d1, d2)
            stroke = 0.12 # Thick stroke
            
            alpha = 0
            if dist < stroke:
                if dist > stroke - 0.05: # Soft edge (1px approx)
                     alpha = (stroke - dist) / 0.05
                else:
                    alpha = 1.0
            
            # Color: Pure Black (0,0,0) with Alpha
            final_a = int(255 * alpha)
            raw_data.extend([0, 0, 0, final_a])
            
    compressed = zlib.compress(bytes(raw_data), 9)
    png_data.extend(create_chunk(b'IDAT', compressed))
    png_data.extend(create_chunk(b'IEND', b''))
    
    with open(filename, 'wb') as f:
        f.write(png_data)
    print(f"Generated {filename}")

# Generate all sizes needed
sizes = [32, 128, 256, 512]
files = ['32x32.png', '128x128.png', '128x128@2x.png', 'icon.png']

for size, name in zip(sizes, files):
    generate_png(f'src-tauri/icons/{name}', size, size)

# Generate Menubar Icon
generate_menubar_icon('src-tauri/icons/icon-menubar.png')

print("All icons regenerated in RGBA format.")
