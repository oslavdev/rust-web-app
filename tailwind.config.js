module.exports = {
    content: [
      "./src/**/*.{html,rs}", 
      "./index.html"
    ],
    theme: {
      extend: {},
    },
    plugins: [
     require('@tailwindcss/typography'),
    ],
  }
