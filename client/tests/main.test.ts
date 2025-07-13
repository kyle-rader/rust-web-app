import { expect, test } from '@playwright/test';

// Global API mocks for all tests
test.beforeEach(async ({ page }) => {
	// Mock /api/status endpoint
	await page.route('**/api/status', async route => {
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				status: 'ok',
				message: 'Server is running',
				timestamp: new Date().toISOString()
			})
		});
	});

	// Mock other common API endpoints with default responses
	await page.route('**/api/**', async route => {
		const url = route.request().url();
		
		// Handle different API endpoints
		if (url.includes('/api/login')) {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					id: 1,
					display_name: 'Test User',
					email: 'test@example.com',
					createdAt: new Date().toISOString(),
					updatedAt: new Date().toISOString()
				})
			});
		} else if (url.includes('/api/register')) {
			await route.fulfill({
				status: 201,
				contentType: 'application/json',
				body: JSON.stringify({
					id: 2,
					display_name: 'New User',
					email: 'newuser@example.com',
					createdAt: new Date().toISOString(),
					updatedAt: new Date().toISOString()
				})
			});
		} else {
			// Default mock for any other /api/* endpoints
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({ message: 'Mock response' })
			});
		}
	});
});

test.describe('Main Application', () => {
	test('index page has expected content', async ({ page }) => {
		await page.goto('/');
		
		// Check main heading
		await expect(page.getByRole('heading', { name: 'RustWebApp' })).toBeVisible();
		
		// Check tagline
		await expect(page.getByText('Build - Deploy - Scale')).toBeVisible();
		
		// Check main action buttons
		await expect(page.getByRole('button', { name: 'Get Started' })).toBeVisible();
		await expect(page.getByRole('button', { name: 'View Docs' })).toBeVisible();
		await expect(page.getByRole('button', { name: 'Browse Examples' })).toBeVisible();
		
		// Check that all sections are present
		await expect(page.getByRole('heading', { name: 'Quick Start' })).toBeVisible();
		await expect(page.getByRole('heading', { name: 'Documentation' })).toBeVisible();
		await expect(page.getByRole('heading', { name: 'Examples' })).toBeVisible();
	});

	test('navigation to login page', async ({ page }) => {
		await page.goto('/');
		
		// Navigate to login page (you might need to add a login link)
		await page.goto('/login');
		
		// Check login page content
		await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
		await expect(page.getByLabel('Email')).toBeVisible();
		await expect(page.getByLabel('Password')).toBeVisible();
		await expect(page.getByRole('button', { name: 'Login' })).toBeVisible();
	});

	test('login form functionality', async ({ page }) => {
		await page.goto('/login');
		
		// Fill login form
		await page.getByLabel('Email').fill('test@example.com');
		await page.getByLabel('Password').fill('password123');
		
		// Check that form fields have correct values
		await expect(page.getByLabel('Email')).toHaveValue('test@example.com');
		await expect(page.getByLabel('Password')).toHaveValue('password123');
		
		// Submit form (this will likely fail without a backend, but we can test the UI)
		await page.getByRole('button', { name: 'Login' }).click();
		
		// The form should be submitted (we can't test the actual login without a backend)
		// But we can verify the form submission doesn't crash the page
		await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
	});

	test('responsive design elements', async ({ page }) => {
		await page.goto('/');
		
		// Test on desktop viewport
		await page.setViewportSize({ width: 1200, height: 800 });
		await expect(page.getByRole('heading', { name: 'RustWebApp' })).toBeVisible();
		
		// Test on mobile viewport
		await page.setViewportSize({ width: 375, height: 667 });
		await expect(page.getByRole('heading', { name: 'RustWebApp' })).toBeVisible();
		
		// Test on tablet viewport
		await page.setViewportSize({ width: 768, height: 1024 });
		await expect(page.getByRole('heading', { name: 'RustWebApp' })).toBeVisible();
	});

	test('form validation on login page', async ({ page }) => {
		await page.goto('/login');
		
		// Try to submit empty form
		await page.getByRole('button', { name: 'Login' }).click();
		
		// Check that required fields are enforced (browser validation)
		const emailInput = page.getByLabel('Email');
		const passwordInput = page.getByLabel('Password');
		
		// These should have required attributes
		await expect(emailInput).toHaveAttribute('required');
		await expect(passwordInput).toHaveAttribute('required');
	});

	test('navigation between pages', async ({ page }) => {
		// Test navigation to different pages
		await page.goto('/');
		await expect(page.getByRole('heading', { name: 'RustWebApp' })).toBeVisible();
		
		await page.goto('/login');
		await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
		
		await page.goto('/register');
		await expect(page.getByRole('heading', { name: 'Register' })).toBeVisible();
		
		await page.goto('/about');
		await expect(page.getByRole('heading', { name: 'About' })).toBeVisible();
	});
});
