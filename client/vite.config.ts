import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

const server_host = process.env.SERVER_HOST || 'localhost';
const server_port = process.env.SERVER_PORT || '3000';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	server: {
		proxy: {
			'/api': `http://${server_host}:${server_port}`
		}
	}
});
