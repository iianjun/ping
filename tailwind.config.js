/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "sans-serif"],
      },
      colors: {
        "primary-100": "#D2D4D5",
        "primary-200": "#B5B7B9",
        "primary-300": "#999C9E",
        "primary-400": "#808688",
        "primary-500": "#6C7275",
        "primary-600": "#55585A",
        "primary-700": "#3A3D41",
        "primary-800": "#232627",
        "primary-900": "#141718",
        "secondary-red": "#EB3440",
        "secondary-blue": "#4A90E2",
        "secondary-purple": "#8A2BE2",
        "secondary-green": "#3FDD78",
      },
      backgroundImage: {
        "selected-gradient":
          "linear-gradient(to left, #323337, rgba(70, 79, 111, 0.3))",
      },
      boxShadow: {
        selected:
          "inset 0px 0.0625rem 0 rgba(255,255,255,0.05), 0 0.25rem 0.5rem 0 rgba(0,0,0,0.1)",
      },
      transitionTimingFunction: {
        smooth: "cubic-bezier(0.4, 0, 0.6, 1)",
      },
      transitionProperty: {
        transform: "transform",
      },
      transitionDuration: {
        600: "600ms",
      },
    },
  },
  plugins: [],
};
