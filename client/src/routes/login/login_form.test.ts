import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import LoginForm from './login_form.svelte';
import { createMockUser } from '../../test/utils';

describe('LoginForm', () => {
	let mockLoginHandler: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		mockLoginHandler = vi.fn();
	});

	it('renders login form with email and password fields', () => {
		render(LoginForm);
		
		expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /login/i })).toBeInTheDocument();
	});

	it('has required attributes on form fields', () => {
		render(LoginForm);
		
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		
		expect(emailInput).toHaveAttribute('type', 'email');
		expect(emailInput).toHaveAttribute('required');
		expect(passwordInput).toHaveAttribute('type', 'password');
		expect(passwordInput).toHaveAttribute('required');
	});

	it('updates form values when user types', async () => {
		render(LoginForm);
		
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'password123' } });
		
		expect(emailInput).toHaveValue('test@example.com');
		expect(passwordInput).toHaveValue('password123');
	});

	it('dispatches login event with form data when submitted', async () => {
		const { component } = render(LoginForm);
		
		// Set up event listener
		component.$on('login', mockLoginHandler);
		
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		const submitButton = screen.getByRole('button', { name: /login/i });
		
		// Fill form
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'password123' } });
		
		// Submit form
		await fireEvent.click(submitButton);
		
		expect(mockLoginHandler).toHaveBeenCalledWith(
			expect.objectContaining({
				detail: {
					email: 'test@example.com',
					password: 'password123'
				}
			})
		);
	});

	it('shows ready state when both fields are filled', async () => {
		render(LoginForm);
		
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		const submitButton = screen.getByRole('button', { name: /login/i });
		
		// Initially button should not have ready class
		expect(submitButton).not.toHaveClass('ready');
		
		// Fill both fields
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'password123' } });
		
		// Button should now have ready class
		expect(submitButton).toHaveClass('ready');
	});

	it('does not show ready state when only one field is filled', async () => {
		render(LoginForm);
		
		const emailInput = screen.getByLabelText(/email/i);
		const submitButton = screen.getByRole('button', { name: /login/i });
		
		// Fill only email
		await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
		
		// Button should not have ready class
		expect(submitButton).not.toHaveClass('ready');
	});

	it('initializes with empty form values', () => {
		render(LoginForm);
		
		const emailInput = screen.getByLabelText(/email/i);
		const passwordInput = screen.getByLabelText(/password/i);
		
		// Form should start with empty values
		expect(emailInput).toHaveValue('');
		expect(passwordInput).toHaveValue('');
	});

	it('prevents submission when form is invalid', async () => {
		const { component } = render(LoginForm);
		
		component.$on('login', mockLoginHandler);
		
		const submitButton = screen.getByRole('button', { name: /login/i });
		
		// Try to submit empty form
		await fireEvent.click(submitButton);
		
		// Since the form has required fields, the browser should prevent submission
		// and no event should be dispatched
		expect(mockLoginHandler).not.toHaveBeenCalled();
	});
}); 