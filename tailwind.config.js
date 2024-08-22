/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",

  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],

  theme: {

    // Define fonts
    fontFamily: {
      inter: ['Inter'],
      faBrands: ['FA-Brands'],
      faRegular: ['FA-Regular'],
      faSolid: ['FA-Solid'],
    },

    extend: {},
  },

  plugins: [],
};
