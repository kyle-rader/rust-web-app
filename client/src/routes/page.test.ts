import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import Page from './+page.svelte';

describe('Main Page', () => {
	it('renders the main heading', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /rustwebapp/i })).toBeInTheDocument();
	});

	it('renders the tagline', () => {
		render(Page);
		
		expect(screen.getByText(/laugh - solve - learn/i)).toBeInTheDocument();
	});

	it('renders the join game section', () => {
		render(Page);
		
		expect(screen.getByPlaceholderText(/code/i)).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /join game/i })).toBeInTheDocument();
	});

	it('renders the host section', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /host/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /create lobby/i })).toBeInTheDocument();
	});

	it('renders the solo play section', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /play solo/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /choose game/i })).toBeInTheDocument();
	});

	it('includes ResponsivePad component', () => {
		const { container } = render(Page);
		
		// ResponsivePad renders a paragraph element
		const responsivePad = container.querySelector('p');
		expect(responsivePad).toBeInTheDocument();
	});

	it('has proper form structure for join game', () => {
		render(Page);
		
		const codeInput = screen.getByPlaceholderText(/code/i);
		const joinButton = screen.getByRole('button', { name: /join game/i });
		
		expect(codeInput).toHaveAttribute('type', 'text');
		expect(joinButton).toBeInTheDocument();
	});

	it('has accessible button labels', () => {
		render(Page);
		
		expect(screen.getByRole('button', { name: /join game/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /create lobby/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /choose game/i })).toBeInTheDocument();
	});

	it('renders all three main options', () => {
		render(Page);
		
		// Should have three option sections
		const options = screen.getAllByRole('button');
		expect(options).toHaveLength(3);
		
		// Check for specific buttons
		expect(screen.getByRole('button', { name: /join game/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /create lobby/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /choose game/i })).toBeInTheDocument();
	});

	it('has proper input field for game code', () => {
		render(Page);
		
		const codeInput = screen.getByPlaceholderText(/code/i);
		expect(codeInput).toBeInTheDocument();
		expect(codeInput).toHaveAttribute('type', 'text');
	});
}); 