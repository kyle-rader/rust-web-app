import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { goto } from '$app/navigation';
import LoginPage from './+page.svelte';
import { createMockUser } from '../../test/utils';
import * as requestsModule from '$lib/requests';
import * as userStoreModule from '$lib/stores/user';

// Mock dependencies
vi.mock('$app/navigation');
vi.mock('$lib/requests');
vi.mock('$lib/stores/user');

describe('Login Page', () => {
	const mockGoto = vi.mocked(goto);
	const mockPost = vi.mocked(requestsModule.post);
	const mockUserStore = vi.mocked(userStoreModule.user);

	beforeEach(() => {
		vi.clearAllMocks();
		mockUserStore.login = vi.fn();
	});

	it('renders login page with title', () => {
		render(LoginPage);
		
		expect(screen.getByRole('heading', { name: /login/i })).toBeInTheDocument();
	});

	it('includes ResponsivePad component', () => {
		const { container } = render(LoginPage);
		
		// ResponsivePad renders a paragraph element
		const responsivePad = container.querySelector('p');
		expect(responsivePad).toBeInTheDocument();
	});

	it('renders LoginForm component', () => {
		render(LoginPage);
		
		// LoginForm should be present (it renders form elements)
		expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /login/i })).toBeInTheDocument();
	});

	it('handles successful login', async () => {
		const mockUser = createMockUser();
		mockPost.mockResolvedValue({
			ok: true,
			json: () => Promise.resolve(mockUser)
		} as Response);

		render(LoginPage);
		
		// Fill form
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'password123' } });
		
		// Submit form
		const submitButton = screen.getByRole('button', { name: /login/i });
		await fireEvent.click(submitButton);
		
		// Wait for async operations
		await waitFor(() => {
			expect(mockPost).toHaveBeenCalledWith('/api/login', {
				email: 'test@example.com',
				password: 'password123'
			});
		});
		
		await waitFor(() => {
			expect(mockUserStore.login).toHaveBeenCalledWith(mockUser);
		});
		
		await waitFor(() => {
			expect(mockGoto).toHaveBeenCalledWith('/');
		});
	});

	it('handles login error', async () => {
		const errorMessage = 'Invalid credentials';
		mockPost.mockResolvedValue({
			ok: false,
			json: () => Promise.resolve({ msg: errorMessage })
		} as Response);

		render(LoginPage);
		
		// Fill form
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'wrongpassword' } });
		
		// Submit form
		const submitButton = screen.getByRole('button', { name: /login/i });
		await fireEvent.click(submitButton);
		
		// Wait for error to appear
		await waitFor(() => {
			expect(screen.getByText(errorMessage)).toBeInTheDocument();
		});
		
		// Should not call user store login or navigation
		expect(mockUserStore.login).not.toHaveBeenCalled();
		expect(mockGoto).not.toHaveBeenCalled();
	});

	it('clears error on successful login after failed attempt', async () => {
		// First, trigger an error
		const errorMessage = 'Invalid credentials';
		mockPost.mockResolvedValueOnce({
			ok: false,
			json: () => Promise.resolve({ msg: errorMessage })
		} as Response);

		render(LoginPage);
		
		// Fill and submit form to trigger error
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'wrongpassword' } });
		
		const submitButton = screen.getByRole('button', { name: /login/i });
		await fireEvent.click(submitButton);
		
		// Wait for error to appear
		await waitFor(() => {
			expect(screen.getByText(errorMessage)).toBeInTheDocument();
		});
		
		// Now trigger success
		const mockUser = createMockUser();
		mockPost.mockResolvedValueOnce({
			ok: true,
			json: () => Promise.resolve(mockUser)
		} as Response);
		
		// Submit form again
		await fireEvent.click(submitButton);
		
		// Error should be cleared
		await waitFor(() => {
			expect(screen.queryByText(errorMessage)).not.toBeInTheDocument();
		});
	});
}); 