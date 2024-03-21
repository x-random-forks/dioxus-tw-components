/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
        extend: {
            borderRadius: {
                "radius": 'var(--radius)',
            },
            colors: {
                "primary": 'hsl(var(--primary))',
                "primary-foreground": 'hsl(var(--primary-foreground))',
                "secondary": 'hsl(var(--secondary))',
                "secondary-foreground": 'hsl(var(--secondary-foreground))',
                "accent": 'hsl(var(--accent))',
                "accent-foreground": 'hsl(var(--accent-foreground))',
                "border": 'hsl(var(--border))',
                "input": 'hsl(var(--input))',
                "ring": 'hsl(var(--ring))',
            }
        },
    },
    plugins: [],
}

