import fs from 'fs';
import { createCanvas } from 'canvas';

// Create a 512x512 canvas
const canvas = createCanvas(512, 512);
const ctx = canvas.getContext('2d');

// Create gradient background
const gradient = ctx.createLinearGradient(0, 0, 512, 512);
gradient.addColorStop(0, '#3B82F6');    // Blue
gradient.addColorStop(1, '#8B5CF6');    // Purple

// Draw rounded rectangle background
const radius = 110;
ctx.fillStyle = gradient;
ctx.beginPath();
ctx.moveTo(radius, 0);
ctx.lineTo(512 - radius, 0);
ctx.quadraticCurveTo(512, 0, 512, radius);
ctx.lineTo(512, 512 - radius);
ctx.quadraticCurveTo(512, 512, 512 - radius, 512);
ctx.lineTo(radius, 512);
ctx.quadraticCurveTo(0, 512, 0, 512 - radius);
ctx.lineTo(0, radius);
ctx.quadraticCurveTo(0, 0, radius, 0);
ctx.closePath();
ctx.fill();

// Draw white circle background for icon
ctx.fillStyle = 'rgba(255, 255, 255, 0.95)';
ctx.beginPath();
ctx.arc(256, 256, 180, 0, Math.PI * 2);
ctx.fill();

// Draw checkmark
ctx.strokeStyle = '#3B82F6';
ctx.lineWidth = 32;
ctx.lineCap = 'round';
ctx.lineJoin = 'round';

ctx.beginPath();
ctx.moveTo(160, 256);
ctx.lineTo(220, 320);
ctx.lineTo(360, 190);
ctx.stroke();

// Draw list lines (right side)
ctx.strokeStyle = '#8B5CF6';
ctx.lineWidth = 16;
ctx.lineCap = 'round';

// Top line
ctx.beginPath();
ctx.moveTo(240, 150);
ctx.lineTo(360, 150);
ctx.stroke();

// Bottom line
ctx.beginPath();
ctx.moveTo(240, 360);
ctx.lineTo(360, 360);
ctx.stroke();

// Save to file
const buffer = canvas.toBuffer('image/png');
fs.writeFileSync('./src-tauri/icons/icon.png', buffer);

console.log('✅ Icon generated successfully at src-tauri/icons/icon.png');

