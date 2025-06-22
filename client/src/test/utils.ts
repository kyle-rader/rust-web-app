import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { vi } from 'vitest';
import type { User } from '$lib/types/user';

// Helper to create a mock user for testing
export function createMockUser(overrides: Partial<User> = {}): User {
	return {
		id: 1,
		display_name: 'Test User',
		email: 'test@example.com',
		createdAt: new Date('2024-01-01'),
		updatedAt: new Date('2024-01-01'),
		...overrides
	};
}

// Helper to render a component with default props
export function renderComponent(Component: any, props: any = {}) {
	return render(Component, props);
}

// Helper to wait for async operations
export async function waitForElement(selector: string) {
	return await waitFor(() => {
		const element = document.querySelector(selector);
		if (!element) {
			throw new Error(`Element with selector "${selector}" not found`);
		}
		return element;
	});
}

// Helper to simulate user interactions
export async function fillForm(fields: Record<string, string>) {
	for (const [name, value] of Object.entries(fields)) {
		const input = screen.getByLabelText(new RegExp(name, 'i'));
		await fireEvent.input(input, { target: { value } });
	}
}

// Helper to submit a form
export async function submitForm() {
	const form = screen.getByRole('button', { name: /submit|login|register/i });
	await fireEvent.click(form);
}

// Mock fetch for API testing
export function mockFetch(response: any, status = 200) {
	return vi.fn().mockResolvedValue({
		ok: status >= 200 && status < 300,
		status,
		json: () => Promise.resolve(response),
		text: () => Promise.resolve(JSON.stringify(response))
	});
} 