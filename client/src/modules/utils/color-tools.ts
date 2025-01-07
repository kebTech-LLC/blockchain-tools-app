// Simulates Java's String.hashCode() method.
function hashCode(s: string): number {
    let hash = 0;
    if (s.length === 0) return hash;
    for (let i = 0; i < s.length; i++) {
        const char = s.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash; // Convert to 32bit integer
    }
    return hash;
}

const hslToHex = (h: number, s: number, l: number): string => {
    h /= 360;
    let r, g, b;

    const hue2rgb = (p, q, t) => {
        if (t < 0) t += 1;
        if (t > 1) t -= 1;
        if (t < 1/6) return p + (q - p) * 6 * t;
        if (t < 1/2) return q;
        if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
        return p;
    };

    if (s === 0) {
        r = g = b = l; // achromatic
    } else {
        const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
        const p = 2 * l - q;
        r = hue2rgb(p, q, h + 1/3);
        g = hue2rgb(p, q, h);
        b = hue2rgb(p, q, h - 1/3);
    }

    const toHex = (x: number) => {
        const hex = Math.round(x * 255).toString(16);
        return hex.length === 1 ? "0" + hex : hex;
    };

    return `#${toHex(r).replace('-', '').substring(0, 2)}${toHex(g).replace('-', '').substring(0, 2)}${toHex(b).replace('-', '').substring(0, 2)}`
};

const hexToHSLO = (hex: string): string => {
    let r = 0, g = 0, b = 0, a = 1;

    // 3 digits
    if (hex.length === 4) {
        r = parseInt(hex[1] + hex[1], 16);
        g = parseInt(hex[2] + hex[2], 16);
        b = parseInt(hex[3] + hex[3], 16);
    }
    // 6 digits
    else if (hex.length === 7) {
        r = parseInt(hex[1] + hex[2], 16);
        g = parseInt(hex[3] + hex[4], 16);
        b = parseInt(hex[5] + hex[6], 16);
    }
    // 8 digits
    else if (hex.length === 9) {
        r = parseInt(hex[1] + hex[2], 16);
        g = parseInt(hex[3] + hex[4], 16);
        b = parseInt(hex[5] + hex[6], 16);
        a = parseInt(hex[7] + hex[8], 16) / 255;
    }

    r /= 255;
    g /= 255;
    b /= 255;

    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    let h = 0, s = 0, l = (max + min) / 2;

    if (max === min) {
        h = s = 0; // achromatic
    } else {
        const d = max - min;
        s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
        switch (max) {
            case r: h = (g - b) / d + (g < b ? 6 : 0); break;
            case g: h = (b - r) / d + 2; break;
            case b: h = (r - g) / d + 4; break;
        }
        h /= 6;
    }

    return `hsl(${h * 360}, ${s * 100}%, ${l * 100}%, ${a})`;
};

function mapTimeToHue(timestamp: number): number {
    const cyclePeriod = 60 * 100; // The period of the sine function cycle, in milliseconds
    // Using Math.sin to get a smoothly transitioning value between -1 and 1
    const sineValue = Math.sin(2 * Math.PI * (timestamp % cyclePeriod) / cyclePeriod);
    // Converting the sineValue from a -1 to 1 range to a 0 to 360 range
    const hue = (sineValue + 1) * 180;
    return hue;
}
  
function getKeyColor(key: number, mode: number, opacity?: number): string {
    // Define lightness values for major and minor
    const majorLightness = 75;
    const minorLightness = 40;

    // Convert key to hue (0-360)
    const hue = key * 360 / 12; // There are 12 keys in music, so each key corresponds to 30 degrees of hue

    // Determine lightness based on mode
    const lightness = mode === 1 ? majorLightness : minorLightness;

    // Return HSL color
    const color = `hsl(${hue}, 100%, ${lightness}%, ${opacity? opacity : 1})`
    return color
}

// Converts a number to an HSL color.
function numberToHSL(i: number, colorStyle: 'pastel' | 'bold' | 'dark' | 'bright' | 'neutral'): string {
    const shortened = i % 360; // Keep it within the 360 degrees of hue
    let saturation: number, lightness: number;
    
    switch(colorStyle) {
        case 'pastel':
            saturation = 30;
            lightness = 80;
            break;
        case 'bold':
            saturation = 80;
            lightness = 50;
            break;
        case 'dark':
            saturation = 70;
            lightness = 20;
            break;
        case 'bright':
            saturation = 90;
            lightness = 60;
            break;
        case 'neutral':
            saturation = 20;
            lightness = 40;
            break;
        default:
            saturation = 50;
            lightness = 50;
            break;
    }
    
    return `hsl(${shortened}, ${saturation}%, ${lightness}%)`;
}
const numberToHex = (h: number, colorStyle: 'pastel' | 'bold' | 'dark' | 'bright' | 'neutral'): string => {
    let saturation: number, lightness: number;
    
    switch(colorStyle) {
        case 'pastel':
            saturation = 0.30;
            lightness = 0.80;
            break;
        case 'bold':
            saturation = 0.80;
            lightness = 0.50;
            break;
        case 'dark':
            saturation = 0.70;
            lightness = 0.20;
            break;
        case 'bright':
            saturation = 0.90;
            lightness = 0.60;
            break;
        case 'neutral':
            saturation = 0.20;
            lightness = 0.40;
            break;
        default:
            saturation = 0.50;
            lightness = 0.50;
            break;
    }
    
    return hslToHex(h, saturation, lightness);
}




// Takes a string and generates a gradient background.
function stringToGradient(s: string, colorCount: number, colorStyle?: 'pastel' | 'bold' | 'dark' | 'bright' | 'neutral'): string {
    const hash = hashCode(s);
    const colors: string[] = [];
    const angle = hash % 360; // Determine angle from hash

    // Generate distinct colors based on the hash
    for (let i = 0; i < colorCount; i++) {
        // Each color is 360/colorCount degrees apart on the color wheel
        colors.push(numberToHSL(hash + i * (360 / colorCount), colorStyle || 'bold'));
    }

    // Return a CSS gradient using the generated colors
    return `linear-gradient(${angle}deg, ${colors.join(', ')})`;
}

const stringToColors = (s: string, colorCount: number, colorStyle?: 'pastel' | 'bold' | 'dark' | 'bright' | 'neutral'): string[] => {
    const hash = hashCode(s) % 360; // Use the hash modulo 360 as our base hue
    const colors: string[] = [];

    for (let i = 0; i < colorCount; i++) {
        // Each color is 360/colorCount degrees apart on the color wheel
        const hue = (hash + i * (360 / colorCount)) % 360;
        colors.push(numberToHex(hue, colorStyle || 'bold'));
    }

    return colors;
}

function stringToSmoothGradient(s: string, colorCount: number, colorStyle?: 'pastel' | 'bold' | 'dark' | 'bright' | 'neutral'): string {
    // Instead of using the raw hash, map it to a 0-360 hue
    const hue = mapTimeToHue(hashCode(s));
    const colors: string[] = [];
    const angle = hue; // Set the gradient angle to be the hue
  
    // Generate distinct colors based on the hue
    for (let i = 0; i < colorCount; i++) {
      // Each color is 360/colorCount degrees apart on the color wheel
      colors.push(numberToHSL(hue + i * (360 / colorCount), colorStyle || 'bold'));
    }
  
    // Return a CSS gradient using the generated colors
    return `linear-gradient(${angle}deg, ${colors.join(', ')})`;
}
  



export { stringToGradient, stringToColors, stringToSmoothGradient, getKeyColor, hexToHSLO, hslToHex }