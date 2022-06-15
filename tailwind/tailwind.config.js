module.exports = {
  purge: {
    enabled: process.env.NODE_ENV === 'production',
    mode: 'all',
    // source_code represents the rust (yew?) source code root
    content: ["./source_code/src/**/*.rs", "./source_code/index.html", "./input/tailwind.css"]
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {colors: {
      "slate": "#3D3D3D",
      "slate-dim": "#222222"
    }},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}

