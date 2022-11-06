const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ["./frontend/src/**/*.{html,rs}"],
  theme: {
    screens: {
      'xs': '400px',
      ...defaultTheme.screens,
    },
    extend: {
      colors: {
        "dark-primary": "#713FFF",
        "dark-secondary": "#ba000d",
        "dark-content-background": "#111827",
        "dark-background": "#0F0F0F",
        "dark-text": "#F1F1F1",
        "dark-hover-bg": "#808080",
        "dark-button-bg": "#272727",
        "dark-border": "#30363D",

        "light-primary": "#2196f3",
        "light-secondary": "#f50057",
        "light-content-background": "#f3f6f4",
        "light-background": "#FFFFFF",
        "light-text": "#0F0F0F",
        "light-hover-bg": "#c0c0c0",
        "light-button-bg": "#DDDDDD",
        "light-border": "#E1E5EA",

        "link-text": "#31A0D3"
      },
      animation: {
        "slide-in-left": "slide-in-left 0.3s cubic-bezier(0.250, 0.460, 0.450, 0.940)   both",
        "slide-out-left": "slide-out-left 0.3s cubic-bezier(0.550, 0.085, 0.680, 0.530)   both"
      },
      keyframes: {
        "slide-in-left": {
          "0%": {
            transform: "translateX(-100%)",
            opacity: "0"
          },
          to: {
            transform: "translateX(0)",
            opacity: "1"
          }
        },
        "slide-out-left": {
          "0%": {
            transform: "translateX(0)",
            opacity: "1"
          },
          to: {
            transform: "translateX(-100%)",
            opacity: "0"
          }
        }
      }
    },

  },
  plugins: [],
  darkMode: "class"
}
