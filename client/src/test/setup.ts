import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock SvelteKit modules that aren't available in test environment
vi.mock('$app/navigation', () => ({
	goto: vi.fn(),
	preloadData: vi.fn(),
	preloadCode: vi.fn(),
	beforeNavigate: vi.fn(),
	afterNavigate: vi.fn(),
	invalidate: vi.fn(),
	invalidateAll: vi.fn()
}));

vi.mock('$app/stores', () => ({
	getStores: vi.fn(),
	navigating: { subscribe: vi.fn() },
	page: { subscribe: vi.fn() },
	updated: { subscribe: vi.fn() }
}));

// Mock browser APIs
Object.defineProperty(window, 'matchMedia', {
	writable: true,
	value: vi.fn().mockImplementation(query => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: vi.fn(), // deprecated
		removeListener: vi.fn(), // deprecated
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn(),
	})),
}); 