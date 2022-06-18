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
      transitionProperty: {
        'load': 'width, opacity',
      },
      gridTemplateColumns: {
        '52': 'grid-template-columns: repeat(52, minmax(0, 1fr));'
      },
      width: {
        "500": "500px",
        "700": "700px"
      },
      colors: {
      "slate": "#3D3D3D",
      "slate-dim": "#222222",
      "moss": "#5B7860",
      "moss-dim": "#1D261F",
      "moss-dark": "#080A08",
      "ice": "#556978",
      "ice-dim": "#1B2226",
      "tint": "#484848",
      "dim": "#707070",
      "dark": "#484848",
      "sponge": "#777855"
    }},
  },
  variants: {
    extend: {
      zIndex: ["hover"],
      outline: ["hover"],
      transitionDuration: ['hover'],
      transitionDelay: ['hover'],
    },
  },
  plugins: [],
}

