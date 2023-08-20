import defaultTheme from 'tailwindcss/defaultTheme';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Figtree', ...defaultTheme.fontFamily.sans],
        'cyber': ['Cyber', 'sans-serif']
      },
      colors: {
        'darker': '#121212',
        'dark': '#1a1a1a',
        'card': '#1e1f22',
      }
    },
  },
  plugins: [],
}