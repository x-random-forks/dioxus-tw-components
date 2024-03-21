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
            colors: {
                "primary": 'hsl(var(--primary))',
                "primary-foreground": 'hsl(var(--primary-foreground))',
                "secondary": 'hsl(var(--secondary))',
                "secondary-foreground": 'hsl(var(--secondary-foreground))',
                "radius": 'var(--radius)',
            }
        },
    },
    plugins: [],
}

