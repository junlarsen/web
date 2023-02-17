const colors = require('@radix-ui/colors');

function toScaleRecord(color) {
  const scale = Object.entries(color).map(([key, value]) => [key.replace(/[A-Za-z]+/, ''), value]);
  return Object.fromEntries(scale);
}

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      fontFamily: {
        noto: 'Noto Sans JP, Noto Sans, sans-serif',
      },
      colors: {
        primary: toScaleRecord(colors.cyan),
        secondary: toScaleRecord(colors.teal),
        gray: toScaleRecord(colors.slate),
      },
    },
  },
};
