import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import tailwind from '@astrojs/tailwind';
import sitemap from '@astrojs/sitemap';
import vercel from '@astrojs/vercel/serverless';

export default defineConfig({
  site: 'https://jun.codes',
  integrations: [mdx(), sitemap(), tailwind()],
  output: 'server',
  adapter: vercel(),
});
