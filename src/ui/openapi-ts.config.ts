import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
	input: 'http://localhost:7878/api-docs/openapi.json',
	output: {
		path: 'src/services/DeckMaster',
		format: 'prettier',
		lint: 'eslint'
	},
	plugins: [
		'@hey-api/schemas',
		{
			dates: true,
			name: '@hey-api/transformers'
		},
		{
			enums: 'javascript',
			name: '@hey-api/typescript'
		},
		{
			name: '@hey-api/sdk',
			transformer: true
		}
	]
});
