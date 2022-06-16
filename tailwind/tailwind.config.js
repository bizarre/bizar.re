module.exports = {
  purge: {
    enabled: process.env.NODE_ENV === 'production',
    mode: 'all',
    // source_code represents the rust (yew?) source code root
    content: ["./source_code/src/**/*.rs", "./source_code/index.html", "./input/tailwind.css"]
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      width: {
        "500": "500px"
      },
      colors: {
      "slate": "#3D3D3D",
      "slate-dim": "#222222",
      "moss": "#5B7860",
      "moss-dim": "#1D261F",
      "ice": "#556978",
      "ice-dim": "#1B2226",
      "dim": "#707070",
      "dark": "#484848",
    }},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}

