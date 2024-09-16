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
    },
  },
  plugins: [],
};
