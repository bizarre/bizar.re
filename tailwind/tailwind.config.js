module.exports = {
  content: {
    enabled: process.env.NODE_ENV === "production",
    mode: "all",
    // source_code represents the rust (yew?) source code root
    content: [
      "./source_code/src/**/*.rs",
      "./source_code/index.html",
      "./input/tailwind.css",
    ],
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      typography(theme) {
        return {
          DEFAULT: {
            // and this is for prose-sm.
            css: {
              color: theme("colors.dim"), // change global color scheme
              code: {
                color: theme("colors.tint"),
              },
              "h1, h2, h3, h4": {
                color: theme("colors.slate"),
              },
              a: {
                // change anchor color and on hover
                color: theme("colors.lavender"),
                "&:hover": {
                  // could be any. It's like extending css selector
                  color: theme("colors.lavende-dark"),
                },
              },
            },
          },
        };
      },
      scale: {
        102: "1.02",
      },
      transitionProperty: {
        load: "width, opacity",
      },
      gridTemplateColumns: {
        52: "grid-template-columns: repeat(52, minmax(0, 1fr));",
      },
      width: {
        500: "500px",
        700: "700px",
      },
      colors: {
        slate: "#3D3D3D",
        "slate-dim": "#222222",
        lavender: "#6A51A0",
        "lavender-dark": "#09080A",
        moss: "#5B7860",
        "moss-dim": "#1D261F",
        "moss-dark": "#080A08",
        ice: "#556978",
        "ice-dim": "#1B2226",
        tint: "#484848",
        dim: "#707070",
        dark: "#090909",
        sponge: "#777855",
        cosmos: "#080312",
      },
    },
  },
  variants: {
    extend: {
      zIndex: ["hover"],
      outline: ["hover"],
      transitionDuration: ["hover"],
      transitionDelay: ["hover"],
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
