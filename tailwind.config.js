module.exports = {
  content: ["./frontend/src/**/*.{html,rs}"],
  theme: {
    extend: {
      colors: {
        "dark-primary": "#713FFF",
        "dark-secondary": "#ba000d",
        "dark-content-background": "#111827",
        "dark-background": "#0F0F0F",
        "dark-text": "#F1F1F1",

        "light-primary": "#2196f3",
        "light-secondary": "#f50057",
        "light-content-background": "#f3f6f4",
        "light-background": "#FFFFFF",
        "light-text": "#0F0F0F",

        "link-text": "#31A0D3"
      }
    },
  },
  plugins: [],
  darkMode: "class"
}
