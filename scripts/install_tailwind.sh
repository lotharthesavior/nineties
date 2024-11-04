
# install dependencies
npm install -D tailwindcss
npx tailwindcss init

# node modules added to gitignore
echo 'node_modules/' >> .gitignore

# create tailwind.css
cat <<EOT > tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [],
}
EOT
