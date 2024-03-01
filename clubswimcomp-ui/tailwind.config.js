/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
  daisyui: {
    themes: [
      {
        corporate: {
          ...require("daisyui/src/theming/themes")["corporate"],
          "primary-content": "FFFFFF",
          "secondary-content": "FFFFFF",
          "error-content": "FFFFFF",
        }
      }
    ],
  }
}

